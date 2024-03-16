<script lang="ts">
    import * as Select from '$lib/components/ui/select';
    import type { Selected } from 'bits-ui';
    import type { Branch } from '../types/repository';

    export let branches: Branch[];

    let selectedBranch: Branch | undefined;

    function onSelect(e?: Selected<Branch>) {
        selectedBranch = e?.value;
        console.log('Selected Branch: ', selectedBranch);
    }
</script>

<div class="flex flex-col border-2 border-solid border-sky-500">
    <div class="self-center">
        <Select.Root onSelectedChange={onSelect}>
            <Select.Trigger class="w-[180px]">
                <Select.Value placeholder="Select a branch" />
            </Select.Trigger>
            <Select.Content>
                <Select.Group>
                    <!-- <Select.Label>Branches</Select.Label> -->
                    {#each branches as branch}
                        <Select.Item value={branch.name} label={branch.name}
                            >{branch.name}</Select.Item
                        >
                    {/each}
                </Select.Group>
            </Select.Content>
            <!-- <Select.Input name="selectedBranch" /> -->
        </Select.Root>
    </div>
    <!-- TODO: FileTree -->
</div>
