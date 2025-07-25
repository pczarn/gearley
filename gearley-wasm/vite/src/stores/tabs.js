import { defineStore } from 'pinia'

export const useTabs = defineStore('tabs', {
  state: () => ({
    tab: ''
  }),
  actions: {
    send(tab) {
      this.tab = tab
    }
  }
})
