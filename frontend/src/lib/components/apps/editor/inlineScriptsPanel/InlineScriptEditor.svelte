<script lang="ts">
	import Button from '$lib/components/common/button/Button.svelte'
	import type { Preview } from '$lib/gen'
	import { createEventDispatcher, getContext, onMount } from 'svelte'
	import type { AppViewerContext, InlineScript } from '../../types'
	import { Maximize2, Trash2 } from 'lucide-svelte'
	import InlineScriptEditorDrawer from './InlineScriptEditorDrawer.svelte'
	import { inferArgs } from '$lib/infer'
	import type { Schema } from '$lib/common'
	import Badge from '$lib/components/common/badge/Badge.svelte'
	import Editor from '$lib/components/Editor.svelte'
	import { defaultIfEmptyString, emptySchema, getModifierKey } from '$lib/utils'
	import { computeFields } from './utils'
	import { deepEqual } from 'fast-equals'
	import type { AppInput } from '../../inputType'
	import Kbd from '$lib/components/common/kbd/Kbd.svelte'
	import SimpleEditor from '$lib/components/SimpleEditor.svelte'
	import { buildExtraLib } from '../../utils'
	import RunButton from './RunButton.svelte'
	import { scriptLangToEditorLang } from '$lib/scripts'
	import ScriptGen from '$lib/components/codeGen/ScriptGen.svelte'
	import DiffEditor from '$lib/components/DiffEditor.svelte'

	let inlineScriptEditorDrawer: InlineScriptEditorDrawer

	export let inlineScript: InlineScript | undefined
	export let name: string | undefined = undefined
	export let id: string
	export let defaultUserInput: boolean = false
	export let fields: Record<string, AppInput> = {}
	export let syncFields: boolean = false
	export let transformer: boolean = false
	export let componentType: string | undefined = undefined

	const { runnableComponents, stateId, worldStore, state, appPath, app } =
		getContext<AppViewerContext>('AppViewerContext')

	export let editor: Editor | undefined = undefined
	let diffEditor: DiffEditor
	let simpleEditor: SimpleEditor
	let validCode = true

	async function inferInlineScriptSchema(
		language: Preview.language,
		content: string,
		schema: Schema
	): Promise<Schema> {
		try {
			await inferArgs(language, content, schema)
			validCode = true
		} catch (e) {
			console.error("Couldn't infer args", e)
			validCode = false
		}

		return schema
	}

	$: inlineScript && (inlineScript.path = `${defaultIfEmptyString(appPath, '')}/${name}`)

	onMount(async () => {
		if (inlineScript && !inlineScript.schema) {
			if (inlineScript.language != 'frontend') {
				inlineScript.schema = await inferInlineScriptSchema(
					inlineScript?.language,
					inlineScript?.content,
					emptySchema()
				)
			}
		}
		if (inlineScript?.schema && inlineScript.language != 'frontend') {
			loadSchemaAndInputsByName()
		}
	})

	const dispatch = createEventDispatcher()
	let runLoading = false

	function preConnect(newFields) {
		if (!componentType) {
			return
		}

		if (componentType === 'steppercomponent') {
			const componentOutputs = $worldStore?.outputsById[id]

			if (componentOutputs.currentStepIndex) {
				newFields['stepIndex'] = {
					type: 'connected',
					connection: {
						componentId: id,
						path: 'currentStepIndex'
					},
					value: componentOutputs.currentStepIndex.peak(),
					fieldType: 'number'
				}
			}
		}
	}

	async function loadSchemaAndInputsByName() {
		if (syncFields && inlineScript) {
			const newSchema = inlineScript.schema ?? emptySchema()
			const hadPreviousFields = Object.keys(fields).length > 0
			const newFields = computeFields(newSchema, defaultUserInput, fields)

			// First time we load the schema, we want to trigger the pre-connect
			if (!hadPreviousFields && Object.keys(newFields).length > 0) {
				preConnect(newFields)
			}

			if (!deepEqual(newFields, fields)) {
				fields = newFields
				$stateId++
			}
		}
	}

	$: extraLib =
		inlineScript?.language == 'frontend' && worldStore
			? buildExtraLib($worldStore?.outputsById ?? {}, id, $state, true)
			: undefined

	let drawerIsOpen: boolean | undefined = undefined
</script>

{#if inlineScript}
	{#if inlineScript.language != 'frontend'}
		<InlineScriptEditorDrawer
			bind:isOpen={drawerIsOpen}
			{editor}
			bind:this={inlineScriptEditorDrawer}
			bind:inlineScript
		/>
	{/if}

	<div class="h-full flex flex-col gap-1">
		<div class="flex justify-between w-full gap-2 px-2 pt-1 flex-row items-center">
			{#if name !== undefined}
				{#if !transformer}
					<input
						on:keydown|stopPropagation
						bind:value={name}
						placeholder="Inline script name"
						class="!text-xs !rounded-xs"
					/>
				{:else}
					<span class="text-xs font-semibold truncate w-full">{name} of {id}</span>
				{/if}
			{/if}
			<div class="flex w-full flex-row gap-2 items-center justify-end">
				{#if validCode}
					<Badge color="green" baseClass="!text-2xs">Valid</Badge>
				{:else}
					<Badge color="red" baseClass="!text-2xs">Invalid</Badge>
				{/if}

				<ScriptGen
					lang={inlineScript?.language}
					editor={inlineScript?.language === 'frontend' ? simpleEditor : editor}
					{diffEditor}
					inlineScript
				/>

				<Button
					title="Delete"
					size="xs"
					color="light"
					btnClasses="!px-2 !bg-red-100 hover:!bg-red-200"
					aria-label="Delete"
					on:click={() => dispatch('delete')}
				>
					<Trash2 size={14} class="text-red-800" />
				</Button>
				{#if inlineScript.language != 'frontend'}
					<Button
						size="xs"
						color="light"
						btnClasses="!px-2 !bg-surface-secondary hover:!bg-surface-hover"
						on:click={() => {
							inlineScriptEditorDrawer?.openDrawer()
						}}
					>
						Full Editor&nbsp;<Maximize2 size={14} />
					</Button>
				{/if}

				<Button
					variant="border"
					size="xs"
					color="light"
					btnClasses="!px-2 !py-1"
					on:click={async () => {
						editor?.format()
						simpleEditor?.format()
					}}
				>
					<div class="flex flex-row gap-1 items-center">
						Format

						<div class="flex flex-row items-center">
							<Kbd small>{getModifierKey()}</Kbd>
							<Kbd small>S</Kbd>
						</div>
					</div>
				</Button>
				<RunButton bind:runLoading {id} inlineScript={!transformer ? inlineScript : undefined} />
			</div>
		</div>

		<!-- {inlineScript.content} -->

		<div class="border-y h-full">
			{#if !drawerIsOpen}
				{#if inlineScript.language != 'frontend'}
					<Editor
						deno={inlineScript.language == 'deno'}
						path={inlineScript.path}
						bind:this={editor}
						class="flex flex-1 grow h-full"
						lang={scriptLangToEditorLang(inlineScript?.language)}
						bind:code={inlineScript.content}
						fixedOverflowWidgets={true}
						cmdEnterAction={async () => {
							if (inlineScript) {
								inlineScript.content = editor?.getCode() ?? ''
							}
							runLoading = true
							await Promise.all($runnableComponents[id]?.cb?.map((f) => f?.(inlineScript)) ?? [])
							runLoading = false
						}}
						on:change={async (e) => {
							if (inlineScript && inlineScript.language != 'frontend') {
								const oldSchema = JSON.stringify(inlineScript.schema)
								if (inlineScript.schema == undefined) {
									inlineScript.schema = emptySchema()
								}
								await inferInlineScriptSchema(inlineScript?.language, e.detail, inlineScript.schema)
								if (JSON.stringify(inlineScript.schema) != oldSchema) {
									inlineScript = inlineScript
									loadSchemaAndInputsByName()
								}
							}
							$app = $app
						}}
					/>
				{:else}
					<SimpleEditor
						bind:this={simpleEditor}
						class="h-full"
						{extraLib}
						bind:code={inlineScript.content}
						lang="javascript"
						cmdEnterAction={async () => {
							runLoading = true
							await await Promise.all(
								$runnableComponents[id]?.cb?.map((f) => f(!transformer ? inlineScript : undefined))
							)
							runLoading = false
						}}
						on:change={() => {
							$app = $app
						}}
					/>
				{/if}

				<DiffEditor
					bind:this={diffEditor}
					class="hidden h-full"
					automaticLayout
					fixedOverflowWidgets
				/>
			{/if}
		</div>
	</div>
{/if}
