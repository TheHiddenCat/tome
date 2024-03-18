<script setup lang="ts">
import { onMounted } from "vue";
import { useSecretStore } from "./stores/secret.store";
import SecretDetails from "./components/SecretDetails.vue";
import SecretsMenu from "./components/SecretsMenu.vue";
import CategoryRail from "./components/CategoryRail.vue";

const secretStore = useSecretStore();

onMounted(async () => {
  await secretStore.listItems();
});
</script>

<template>
  <v-app>
    <v-layout>
      <CategoryRail />
      <SecretsMenu />
      <v-main>
        <SecretDetails></SecretDetails>
      </v-main>
    </v-layout>
  </v-app>
</template>

<style scoped></style>
