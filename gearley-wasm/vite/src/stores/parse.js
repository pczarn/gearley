import { defineStore } from 'pinia'

export const useParse = defineStore('parse', {
  state: () => ({
    result: ''
  }),
  actions: {
    setResult(result) {
      this.result = result
    }
  },
  getters: {
    logs: (state) => {

    }
  }
})
