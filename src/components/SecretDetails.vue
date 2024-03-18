<script setup lang="ts">
import { useSecretStore } from "../stores/secret.store";

const secretStore = useSecretStore();

async function decryptSecret() {
  if (secretStore.passphrase && secretStore.selectedSecret) {
    await secretStore.decryptItem(
      secretStore.passphrase as string,
      secretStore.selectedSecret.category,
      secretStore.selectedSecret.key
    );
    secretStore.passphrase = undefined;
  }
}
</script>

<template>
  <div v-if="secretStore.selectedSecret">
    <p class="pa-4 pb-0 text-h6">{{ secretStore.selectedSecret.key }}</p>
    <v-text-field
      v-model="secretStore.passphrase"
      :disabled="secretStore.isDecrypting"
      type="password"
      prepend-inner-icon="mdi-key"
      label="Enter passphrase"
      class="pa-4 pb-0"
      @keydown.enter.prevent="decryptSecret"
    ></v-text-field>
    <v-textarea
      v-model="secretStore.decryptedSecret"
      :disabled="secretStore.isDecrypting"
      :loading="secretStore.isDecrypting"
      class="pa-4 pt-0"
      prepend-inner-icon="mdi-lock"
      label="Your secret"
      auto-grow
      readonly
    ></v-textarea>
  </div>
  <div v-else>
    <p class="text-h6 mt-12 text-center">
      <v-icon size="small">mdi-key</v-icon> Select or create a secret
    </p>
  </div>
</template>
