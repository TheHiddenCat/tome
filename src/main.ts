import { createApp } from "vue";
import { createPinia } from 'pinia';

import '@mdi/font/css/materialdesignicons.css'
import vuetify from './plugins/vuetify.ts';

import "./styles.css";
import App from "./App.vue";

const pinia = createPinia();
const app = createApp(App);

app.use(vuetify);
app.use(pinia);

app.mount('#app');
