<script lang="ts">
	import { Button } from '../common'

	import { SUPPORTED_LANGUAGES, fixScript } from './lib'
	import type { SupportedLanguage } from '$lib/common'
	import { sendUserToast } from '$lib/toast'
	import type Editor from '../Editor.svelte'
	import { faCheck, faClose, faMagicWandSparkles } from '@fortawesome/free-solid-svg-icons'
	import { dbSchema, existsOpenaiResourcePath } from '$lib/stores'
	import type DiffEditor from '../DiffEditor.svelte'
	import { scriptLangToEditorLang } from '$lib/scripts'
	import Popover from '../Popover.svelte'

	// props
	export let lang: SupportedLanguage
	export let editor: Editor | undefined
	export let diffEditor: DiffEditor | undefined
	export let error: string

	// state
	let genLoading: boolean = false
	let generatedCode = ''
	let explanation = ''

	async function onFix() {
		if (!error) {
			return
		}
		try {
			// close popup ^^
			const elem = document.activeElement as HTMLElement
			if (elem.blur) {
				elem.blur()
			}

			genLoading = true
			const result = await fixScript({
				language: lang,
				code: editor?.getCode() || '',
				error,
				dbSchema: $dbSchema
			})
			generatedCode = result.code
			explanation = result.explanation
		} catch (err) {
			sendUserToast('Failed to generate code', true)
			console.error(err)
		} finally {
			genLoading = false
		}
	}

	function acceptDiff() {
		editor?.setCode(diffEditor?.getModified() || '')
		editor?.format()
		generatedCode = ''
		explanation = ''
		error = ''
	}

	function rejectDiff() {
		generatedCode = ''
		explanation = ''
	}

	function showDiff() {
		diffEditor?.setDiff(editor?.getCode() || '', generatedCode, scriptLangToEditorLang(lang))
		diffEditor?.show()
		editor?.hide()
	}

	function hideDiff() {
		editor?.show()
		diffEditor?.hide()
	}

	$: lang && (generatedCode = '')

	$: generatedCode && showDiff()
	$: !generatedCode && hideDiff()
</script>

{#if error}
	{#if existsOpenaiResourcePath && SUPPORTED_LANGUAGES.has(lang)}
		<div class="mt-2">
			{#if generatedCode}
				<div class="flex gap-1">
					<Button
						title="Discard generated code"
						size="xs"
						color="red"
						spacingSize="xs2"
						on:click={rejectDiff}
						variant="contained"
						startIcon={{ icon: faClose }}
					>
						Discard
					</Button><Button
						title="Accept generated code"
						size="xs"
						color="green"
						spacingSize="xs2"
						on:click={acceptDiff}
						startIcon={{ icon: faCheck }}
					>
						Accept
					</Button>
					{#if explanation}
						<Popover>
							<svelte:fragment slot="text">{explanation}</svelte:fragment>
							<Button size="xs" color="light" variant="contained" spacingSize="xs2">Explain</Button
							></Popover
						>
					{/if}
				</div>
			{:else}
				<Button
					title="Generate code from prompt"
					size="xs"
					color="blue"
					spacingSize="xs2"
					startIcon={{ icon: faMagicWandSparkles }}
					loading={genLoading}
					on:click={onFix}
				>
					AI Fix
				</Button>
			{/if}</div
		>
	{/if}
{/if}
