<script setup lang="ts">
import { ref } from "vue";
import { useCategoryStore } from "../stores/category.store";

const categoryStore = useCategoryStore();

const rail = ref(true);

</script>

<template>
  <v-navigation-drawer
    :rail="rail"
    permanent
    color="black"
  >
    <v-list-item class="text-center" nav>
      <v-icon>mdi-book-open-page-variant</v-icon>
    </v-list-item>
    <v-divider></v-divider>
    <v-list density="compact" nav>
      <v-list-item
        v-for="category in categoryStore.categories"
        :key="category.value"
        :prepend-icon="category.icon"
        :value="category.value"
        :title="category.title"
        color="primary"
      ></v-list-item>
    </v-list>
    <template v-slot:append>
      <v-btn variant="text" icon="mdi-chevron-double-right" @click.stop="rail = !rail"></v-btn>
    </template>
  </v-navigation-drawer>
</template>
