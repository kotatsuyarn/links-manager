import { writable } from 'svelte/store';

const { subscribe, set } = writable(undefined);

export default {
  subscribe,
  show: {
    async settings() {
      set((await import('./components/Settings')).Settings);
    },
    async createList() {
      set((await import('./components/CreateList')).CreateList);
    }
  },
  close() {
    set(undefined);
  },
};