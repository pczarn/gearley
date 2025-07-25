import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createPinia } from 'pinia'
import PrimeVue from 'primevue/config';

import Material from '@primevue/themes/material'
// import 'primevue/resources/primevue.min.css'

import init from "@/assets/pkg/gearley_wasm.js";

await init().catch(console.error);

const pinia = createPinia()

createApp(App).use(pinia).use(PrimeVue, { theme: { preset: Material } }).mount('#app')
