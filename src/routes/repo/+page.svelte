<script lang="ts">
    import { goto } from '$app/navigation';
    import Button from '$lib/components/ui/button/button.svelte';
    import { selectedRepo } from '$lib/stores';
    import { invoke } from '@tauri-apps/api';

    $: repoPath = $selectedRepo;

    function goHome() {
        goto('/');
    }

    let branches: Branch[] = [];
    async function loadGitData() {
        try {
            let repo: Repository = await invoke('load_repo', { path: repoPath });
            branches = repo.branches;
            console.log(branches);
        } catch (e) {
            console.log('Got an error: ', e);
        }
    }

    type Branch = {
        name: string;
        tip_commit_id: string;
    };

    type Repository = {
        branches: Branch[];
    };
</script>

<Button on:click={goHome}>Home</Button>
<p>Repo path: {repoPath}</p>
<Button on:click={loadGitData}>Load some git data</Button>
<ul>
    {#each branches as branch}
        <li>
            <p>{branch.name} | {branch.tip_commit_id}</p>
        </li>
    {/each}
</ul>
