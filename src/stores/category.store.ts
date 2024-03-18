import { Ref, ref } from "vue";
import { defineStore } from "pinia";
import Category from "../interfaces/category.interface";

export const useCategoryStore = defineStore('categories', () => {
  const selectedCategory: Ref<Category | undefined> = ref(undefined);
  const categories: Ref<Array<Category>> = ref(
    [
      {
        title: "Gaming",
        icon: "mdi-controller",
        value: "gaming"
      },
      {
        title: "Socials",
        icon: "mdi-forum",
        value: "socials"
      },
      {
        title: "Emails",
        icon: "mdi-email",
        value: "emails"
      },
      {
        title: "Work",
        icon: "mdi-briefcase",
        value: "work"
      },
      {
        title: "Shopping",
        icon: "mdi-store",
        value: "shopping"
      },
      {
        title: "Banking",
        icon: "mdi-bank",
        value: "banking"
      },
      {
        title: "Websites",
        icon: "mdi-web",
        value: "websites"
      },
      {
        title: "Other",
        icon: "mdi-key-variant",
        value: "other"
      },
    ]
  );
  return { categories, selectedCategory }
});