<script setup lang="ts">
import { Ref, ref, watch } from "vue";
import { useSecretStore } from "../stores/secret.store";
import { useCategoryStore } from "../stores/category.store";

const secretStore = useSecretStore();
const categoryStore = useCategoryStore();

const valid: Ref<boolean> = ref(false);
const dialog: Ref<boolean> = ref(false);
const secret: Ref<string | undefined> = ref(undefined);
const secretRules = [
  (value: string) => {
    if (value) return true;

    return "Secret is required.";
  },
];
const passphrase: Ref<string | undefined> = ref(undefined);
const passphraseRules = [
  (value: string) => {
    if (value) return true;

    return "Passphrase is required.";
  },
  (value: string) => {
    if (value?.length <= 64) return true;

    return "Passphrase must be less than 64 characters.";
  },
];
const category: Ref<string | undefined> = ref(undefined);
const categoryRules = [
  (value: string) => {
    if (value) return true;

    return "Category is required.";
  },
  (value: string) => {
    if (value?.length <= 64) return true;

    return "Category must be less than 64 characters.";
  },
];

const name: Ref<string | undefined> = ref(undefined);
const nameRules = [
  (value: string) => {
    if (value) return true;

    return "Name is required.";
  },
  (value: string) => {
    if (value?.length <= 64) return true;

    return "Name must be less than 64 characters.";
  },
];

async function createSecret() {
  if (valid) {
    await secretStore.encryptItem(
      passphrase.value as string,
      category.value as string,
      name.value as string,
      secret.value as string
    );
    await secretStore.listItems();
    dialog.value = false;
  }
}

watch(dialog, (newDialog: boolean) => {
    if (!newDialog) {
        valid.value = false;
        name.value = undefined;
        category.value = undefined;
        passphrase.value = undefined;
        secret.value = undefined;
    }
});

</script>

<template>
  <v-dialog
    v-model="dialog"
    transition="dialog-bottom-transition"
    max-width="500"
  >
    <template v-slot:activator="{ props: activatorProps }">
      <v-btn
        v-bind="activatorProps"
        block
        variant="flat"
        color="blue-grey-lighten-1"
        class="mt-4"
      >
        <v-icon>mdi-plus</v-icon>
        New
      </v-btn>
    </template>

    <v-card>
      <v-toolbar color="black" title="New secret"></v-toolbar>
      <v-card-text>
        <v-form v-model="valid">
          <v-text-field
            v-model="name"
            label="Name"
            required
            :rules="nameRules"
          ></v-text-field>
          <v-select
            v-model="category"
            label="Category"
            required
            :items="categoryStore.categories"
            item-title="title"
            item-value="value"
            :rules="categoryRules"
          ></v-select>
          <v-text-field
            v-model="passphrase"
            type="password"
            prepend-inner-icon="mdi-key"
            label="Passphrase"
            :rules="passphraseRules"
            required
          ></v-text-field>
          <v-textarea
            v-model="secret"
            prepend-inner-icon="mdi-lock"
            label="Your secret"
            variant="filled"
            auto-grow
            required
            :rules="secretRules"
          ></v-textarea>
        </v-form>
      </v-card-text>

      <v-divider></v-divider>

      <v-card-actions class="justify-end">
        <v-spacer></v-spacer>
        <v-btn text="Close" @click="dialog = false"></v-btn>
        <v-btn
          type="submit"
          :disabled="!valid"
          text="Save"
          variant="flat"
          color="blue-grey-lighten-1"
          @click="createSecret"
        ></v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>
