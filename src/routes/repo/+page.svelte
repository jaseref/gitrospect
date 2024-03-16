<script lang="ts">
    import { selectedRepo } from '$lib/stores';
    import * as Resizable from '$lib/components/ui/resizable';
    import FileTree from '$lib/components/FileTree.svelte';
</script>

<div class="min-h-screen p-8">
    <Resizable.PaneGroup direction="horizontal">
        <Resizable.Pane>
            <FileTree branches={$selectedRepo.branches} />
        </Resizable.Pane>
        <Resizable.Handle />
        <Resizable.Pane>
            <!-- TODO: File Viewer -->
        </Resizable.Pane>
    </Resizable.PaneGroup>
</div>
