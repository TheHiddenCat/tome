<script setup lang="ts">
import { useSecretStore } from "../stores/secret.store";
import Secret from "../interfaces/secret.interface";
import NewItemDialog from "./NewItemDialog.vue";

const secretStore = useSecretStore();

function selectSecret(secret: Secret) {
  secretStore.selectedSecret = secret;
  secretStore.decryptedSecret = undefined;
  secretStore.passphrase = undefined;
}

</script>

<template>
  <v-navigation-drawer permanent color="blue-grey-darken-4">
    <v-autocomplete label="Search" density="compact"></v-autocomplete>
    <v-list density="compact">
      <v-list-item
        v-for="secret in secretStore.secrets"
        :key="secret.key"
        :title="secret.key"
        :subtitle="secret.category"
        :value="secret.key.toLowerCase()"
        @click="selectSecret(secret)"
      >
      </v-list-item>
      <v-list-item>
        <NewItemDialog />
      </v-list-item>
    </v-list>
  </v-navigation-drawer>
</template>
