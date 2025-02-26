<script lang="ts">
	import CenteredPage from '$lib/components/CenteredPage.svelte'
	import { Alert, Badge, Button, Skeleton, Tab, Tabs } from '$lib/components/common'
	import ConfirmationModal from '$lib/components/common/confirmationModal/ConfirmationModal.svelte'
	import DeployWorkspaceDrawer from '$lib/components/DeployWorkspaceDrawer.svelte'
	import Dropdown from '$lib/components/Dropdown.svelte'
	import ListFilters from '$lib/components/home/ListFilters.svelte'
	import PageHeader from '$lib/components/PageHeader.svelte'
	import Popover from '$lib/components/Popover.svelte'
	import SearchItems from '$lib/components/SearchItems.svelte'
	import SharedBadge from '$lib/components/SharedBadge.svelte'
	import ShareModal from '$lib/components/ShareModal.svelte'
	import TableCustom from '$lib/components/TableCustom.svelte'
	import TableSimple from '$lib/components/TableSimple.svelte'
	import Tooltip from '$lib/components/Tooltip.svelte'
	import VariableEditor from '$lib/components/VariableEditor.svelte'
	import type { ContextualVariable, ListableVariable } from '$lib/gen'
	import { OauthService, VariableService } from '$lib/gen'
	import { userStore, workspaceStore } from '$lib/stores'
	import { sendUserToast } from '$lib/toast'
	import { canWrite, isOwner, truncate } from '$lib/utils'
	import {
		faChain,
		faCircle,
		faEdit,
		faEyeSlash,
		faFileExport,
		faPlus,
		faRefresh,
		faShare,
		faTrash
	} from '@fortawesome/free-solid-svg-icons'
	import { Building, DollarSign } from 'lucide-svelte'
	import Icon from 'svelte-awesome'

	type ListableVariableW = ListableVariable & { canWrite: boolean }

	let filter = ''
	let variables: ListableVariableW[] | undefined = undefined
	let filteredItems: (ListableVariableW & { marked?: string })[] | undefined = undefined
	let contextualVariables: ContextualVariable[] = []
	let shareModal: ShareModal
	let variableEditor: VariableEditor
	let loading = {
		contextual: true
	}

	let deleteConfirmedCallback: (() => void) | undefined = undefined
	$: open = Boolean(deleteConfirmedCallback)

	$: owners = Array.from(
		new Set(filteredItems?.map((x) => x.path.split('/').slice(0, 2).join('/')) ?? [])
	).sort()

	let ownerFilter: string | undefined = undefined

	$: if ($workspaceStore) {
		ownerFilter = undefined
	}

	$: preFilteredItems =
		ownerFilter == undefined
			? variables
			: variables?.filter((x) => x.path.startsWith(ownerFilter ?? ''))

	// If relative, the dropdown is positioned relative to its button
	async function loadVariables(): Promise<void> {
		variables = (await VariableService.listVariable({ workspace: $workspaceStore! })).map((x) => {
			return {
				canWrite: canWrite(x.path, x.extra_perms!, $userStore) && x.workspace_id == $workspaceStore,
				...x
			}
		})
	}

	async function loadContextualVariables(): Promise<void> {
		contextualVariables = await VariableService.listContextualVariables({
			workspace: $workspaceStore!
		})
		loading.contextual = false
	}

	async function deleteVariable(path: string, account?: number): Promise<void> {
		if (account) {
			OauthService.disconnectAccount({ workspace: $workspaceStore!, id: account })
		}
		await VariableService.deleteVariable({ workspace: $workspaceStore!, path })
		loadVariables()
		sendUserToast(`Variable ${path} was deleted`)
	}

	$: {
		if ($workspaceStore && $userStore) {
			loadVariables()
			loadContextualVariables()
		}
	}
	let tab: 'workspace' | 'contextual' = 'workspace'

	let deploymentDrawer: DeployWorkspaceDrawer
</script>

<DeployWorkspaceDrawer bind:this={deploymentDrawer} />

<SearchItems
	{filter}
	items={preFilteredItems}
	bind:filteredItems
	f={(x) => x.path + ' ' + x.description}
/>

<CenteredPage>
	<PageHeader
		title="Variables"
		tooltip="Save and permission strings to be reused in Scripts and Flows."
		documentationLink="https://www.windmill.dev/docs/core_concepts/variables_and_secrets"
	>
		<div class="flex flex-row justify-end">
			<Button size="md" startIcon={{ icon: faPlus }} on:click={() => variableEditor.initNew()}>
				New&nbsp;variable
			</Button>
		</div>
	</PageHeader>

	<VariableEditor bind:this={variableEditor} on:create={loadVariables} />
	<ShareModal
		bind:this={shareModal}
		on:change={() => {
			loadVariables()
		}}
	/>

	<Tabs bind:selected={tab}>
		<Tab size="md" value="workspace">
			<div class="flex gap-2 items-center my-1">
				<Building size={18} />
				Workspace
			</div>
		</Tab>
		<Tab size="md" value="contextual">
			<div class="flex gap-2 items-center my-1">
				<DollarSign size={18} />
				Contextual <Tooltip
					documentationLink="https://www.windmill.dev/docs/core_concepts/variables_and_secrets#contextual-variables"
				>
					Contextual variables are passed as environment variables when running a script and depends
					on the execution context.</Tooltip
				>
			</div>
		</Tab>
	</Tabs>
	{#if tab == 'workspace'}
		<div class="pt-2">
			<input placeholder="Search Variable" bind:value={filter} class="input mt-1" />
		</div>
		<ListFilters bind:selectedFilter={ownerFilter} filters={owners} />

		<div class="relative overflow-x-auto pb-40 pr-4">
			{#if !filteredItems}
				<Skeleton layout={[0.5, [2], 1]} />
				{#each new Array(3) as _}
					<Skeleton layout={[[3.5], 0.5]} />
				{/each}
			{:else}
				<TableCustom>
					<tr slot="header-row">
						<th class="!px-0" />
						<th>Path</th>
						<th>Value</th>
						<th>Description</th>
						<th />
						<th />
					</tr>
					<tbody slot="body">
						{#each filteredItems as { path, value, is_secret, description, extra_perms, canWrite, account, is_refreshed, is_expired, refresh_error, is_linked, marked }}
							<tr>
								<td class="!px-0 text-center">
									<SharedBadge {canWrite} extraPerms={extra_perms} />
								</td>
								<td>
									<a
										class="break-all"
										id="edit-{path}"
										on:click={() => variableEditor.editVariable(path)}
										href="#{path}"
									>
										{#if marked}{@html marked}{:else}{path}{/if}
									</a>
								</td>
								<td>
									<span class="inline-flex flex-row">
										<span class="text-sm break-words">
											{truncate(value ?? '****', 20)}
										</span>
										{#if is_secret}
											<Popover notClickable>
												<Icon
													label="Secret"
													class="text-secondary mb-2 ml-2"
													data={faEyeSlash}
													scale={0.8}
												/>
												<span slot="text">This item is secret</span>
											</Popover>
										{/if}
									</span>
								</td>
								<td class="break-words"
									><span class="text-xs text-tertiary">{truncate(description ?? '', 50)}</span></td
								>

								<td class="text-center">
									<div class="flex flex-row">
										<div class="w-10">
											{#if is_linked}
												<Popover notClickable>
													<Icon data={faChain} />
													<div slot="text">
														This variable is linked with a resource of the same path. They are
														deleted and renamed together.
													</div>
												</Popover>
											{/if}
										</div>
										<div class="w-10">
											{#if account}
												<Popover notClickable>
													<Icon data={faRefresh} />
													<div slot="text">
														This OAuth token will be kept up-to-date in the background by Windmill
														using its refresh token
													</div>
												</Popover>
											{/if}
										</div>

										{#if is_refreshed}
											<div class="w-10">
												{#if refresh_error}
													<Popover notClickable>
														<span class="flex h-4 w-4">
															<Icon
																class="text-red-600 animate-ping absolute inline-flex "
																data={faCircle}
																scale={0.7}
																label="Error during exchange of the refresh token"
															/>
															<Icon
																class="text-red-600 relative inline-flex"
																data={faCircle}
																scale={0.7}
																label="Error during exchange of the refresh token"
															/>
														</span>
														<div slot="text">
															Latest exchange of the refresh token did not succeed. Error: {refresh_error}
														</div>
													</Popover>
												{:else if is_expired}
													<Popover notClickable>
														<Icon
															class="text-yellow-600 animate-[pulse_5s_linear_infinite]"
															data={faCircle}
															scale={0.7}
															label="Variable is expired"
														/>
														<div slot="text">
															The access_token is expired, it will get renewed the next time this
															variable is fetched or you can request is to be refreshed in the
															dropdown on the right.
														</div>
													</Popover>
												{:else}
													<Popover notClickable>
														<Icon
															class="text-green-600 animate-[pulse_5s_linear_infinite]"
															data={faCircle}
															scale={0.7}
															label="Variable is tied to an OAuth app"
														/>
														<div slot="text">
															The variable was connected through OAuth and the token is not expired.
														</div>
													</Popover>
												{/if}
											</div>
										{/if}
									</div>
								</td>
								<td>
									<Dropdown
										placement="bottom-end"
										dropdownItems={() => {
											let owner = isOwner(path, $userStore, $workspaceStore)
											return [
												{
													displayName: 'Edit',
													icon: faEdit,
													action: () => variableEditor.editVariable(path),
													disabled: !canWrite
												},
												{
													displayName: 'Delete',
													icon: faTrash,
													type: 'delete',
													action: (event) => {
														if (event?.shiftKey) {
															deleteVariable(path, account)
														} else {
															deleteConfirmedCallback = () => {
																deleteVariable(path, account)
															}
														}
													},
													disabled: !owner
												},
												{
													displayName: 'Deploy to prod/staging',
													icon: faFileExport,
													action: () => {
														deploymentDrawer.openDrawer(path, 'variable')
													}
												},
												{
													displayName: owner ? 'Share' : 'See Permissions',
													action: () => {
														shareModal.openDrawer(path, 'variable')
													},
													icon: faShare
												},
												...(account != undefined
													? [
															{
																displayName: 'Refresh token',
																icon: faRefresh,
																action: async () => {
																	await OauthService.refreshToken({
																		workspace: $workspaceStore ?? '',
																		id: account ?? 0,
																		requestBody: {
																			path
																		}
																	})
																	sendUserToast('Token refreshed')
																	loadVariables()
																}
															}
													  ]
													: [])
											]
										}}
									/>
								</td>
							</tr>
						{/each}
					</tbody>
				</TableCustom>
			{/if}
		</div>
	{:else if tab == 'contextual'}
		<div class="overflow-auto">
			{#if loading.contextual}
				<Skeleton layout={[0.5, [2], 1]} />
				{#each new Array(8) as _}
					<Skeleton layout={[[2.8], 0.5]} />
				{/each}
			{:else}
				<TableSimple
					headers={['name', 'example of value', 'description']}
					data={contextualVariables}
					keys={['name', 'value', 'description']}
					twTextSize="text-sm"
				/>
			{/if}
		</div>
	{/if}
</CenteredPage>

<ConfirmationModal
	{open}
	title="Remove variable"
	confirmationText="Remove"
	on:canceled={() => {
		deleteConfirmedCallback = undefined
	}}
	on:confirmed={() => {
		if (deleteConfirmedCallback) {
			deleteConfirmedCallback()
		}
		deleteConfirmedCallback = undefined
	}}
>
	<div class="flex flex-col w-full space-y-4">
		<span>Are you sure you want to remove this variable?</span>
		<Alert type="info" title="Bypass confirmation">
			<div>
				You can press
				<Badge color="dark-gray">SHIFT</Badge>
				while removing a variable to bypass confirmation.
			</div>
		</Alert>
	</div>
</ConfirmationModal>
