<script lang="ts">
	import { getContext } from 'svelte'
	import { twMerge } from 'tailwind-merge'
	import { initOutput, selectId } from '../../editor/appUtils'
	import type {
		AppViewerContext,
		ComponentCustomCSS,
		ListContext,
		ListInputs,
		RichConfigurations
	} from '../../types'
	import { concatCustomCss } from '../../utils'
	import AlignWrapper from '../helpers/AlignWrapper.svelte'
	import InputValue from '../helpers/InputValue.svelte'
	import InitializeComponent from '../helpers/InitializeComponent.svelte'

	export let id: string
	export let configuration: RichConfigurations
	export let inputType = 'text'
	export let verticalAlignment: 'top' | 'center' | 'bottom' | undefined = undefined
	export let customCss: ComponentCustomCSS<'textinputcomponent'> | undefined = undefined
	export let appCssKey:
		| 'textinputcomponent'
		| 'passwordinputcomponent'
		| 'emailinputcomponent'
		| 'textareainputcomponent' = 'textinputcomponent'
	export let render: boolean

	const { app, worldStore, selectedComponent, connectingInput, componentControl } =
		getContext<AppViewerContext>('AppViewerContext')

	const iterContext = getContext<ListContext>('ListWrapperContext')
	const listInputs: ListInputs | undefined = getContext<ListInputs>('ListInputs')

	let placeholder: string | undefined = undefined
	let defaultValue: string | undefined = undefined
	let value: string | undefined = undefined

	let outputs = initOutput($worldStore, id, {
		result: ''
	})

	$componentControl[id] = {
		setValue(nvalue: string) {
			value = nvalue
		}
	}

	$: handleDefault(defaultValue)

	$: {
		let val = value ?? ''
		outputs?.result.set(val)
		if (iterContext && listInputs) {
			listInputs(id, val)
		}
	}

	function handleDefault(defaultValue: string | undefined) {
		value = defaultValue
	}

	$: css = concatCustomCss($app.css?.[appCssKey], customCss)
</script>

<InputValue {id} input={configuration.placeholder} bind:value={placeholder} />
<InputValue {id} input={configuration.defaultValue} bind:value={defaultValue} />

<InitializeComponent {id} />
{#if render}
	{#if inputType === 'textarea'}
		<textarea
			class={twMerge(
				'windmillapp w-full h-full py-1.5 text-sm focus:ring-indigo-100 px-2 ',
				css?.input?.class ?? ''
			)}
			style="resize:none; {css?.input?.style ?? ''}"
			on:pointerdown|stopPropagation={(e) =>
				!$connectingInput.opened && selectId(e, id, selectedComponent, $app)}
			on:keydown|stopPropagation
			bind:value
			{placeholder}
		/>
	{:else}
		<AlignWrapper {render} {verticalAlignment}>
			{#if inputType === 'password'}
				<input
					class={twMerge(
						'windmillapp w-full py-1.5 text-sm focus:ring-indigo-100 px-2 ',
						css?.input?.class ?? ''
					)}
					style={css?.input?.style ?? ''}
					on:pointerdown|stopPropagation={(e) =>
						!$connectingInput.opened && selectId(e, id, selectedComponent, $app)}
					on:keydown|stopPropagation
					type="password"
					bind:value
					{placeholder}
				/>
			{:else if inputType === 'text'}
				<input
					class={twMerge(
						'windmillapp w-full py-1.5 text-sm focus:ring-indigo-100 px-2 ',
						css?.input?.class ?? ''
					)}
					style={css?.input?.style ?? ''}
					on:pointerdown|stopPropagation={(e) =>
						!$connectingInput.opened && selectId(e, id, selectedComponent, $app)}
					on:keydown|stopPropagation
					type="text"
					bind:value
					{placeholder}
				/>
			{:else if inputType === 'email'}
				<input
					class={twMerge(
						'windmillapp w-full py-1.5 text-sm focus:ring-indigo-100 px-2 ',
						css?.input?.class ?? ''
					)}
					style={css?.input?.style ?? ''}
					on:pointerdown|stopPropagation={(e) =>
						!$connectingInput.opened && selectId(e, id, selectedComponent, $app)}
					on:keydown|stopPropagation
					type="email"
					bind:value
					{placeholder}
				/>
			{/if}
		</AlignWrapper>
	{/if}
{/if}
