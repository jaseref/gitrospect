<script lang="ts">
    import { Button } from '$lib/components/ui/button';
    import { open } from '@tauri-apps/api/dialog';
    import { selectedRepo } from '$lib/stores';
    import { goto } from '$app/navigation';
    import type { Repository } from '$lib/types/repository';
    import { invoke } from '@tauri-apps/api';

    async function onClick() {
        const folderPath = await open({
            title: 'Select a repository',
            directory: true,
            multiple: false
        });

        if (typeof folderPath === 'string') {
            console.log('Got path: ', folderPath);
            try {
                let repo: Repository = await invoke('load_repo', { path: folderPath });
                console.log('Got repo: ', repo);
                selectedRepo.set(repo);
                goto('/repo');
            } catch (e) {
                // TODO: Display error banner
                console.error('Backend Error: ', e);
            }
        }
    }
</script>

<div class="flex min-h-screen flex-col items-center justify-center">
    <h1 class="absolute top-0 mt-14">Gitrospect</h1>
    <div class="mt-24 flex flex-col gap-5">
        <p>Open a repository</p>
        <Button on:click={onClick}>Open</Button>
    </div>
</div>
