import { writable } from 'svelte/store';
import type { Repository } from './types/repository';

export const selectedRepo = writable<Repository>();
