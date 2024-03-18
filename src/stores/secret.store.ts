import { Ref, ref } from "vue";
import { defineStore } from "pinia";
import { invoke } from "@tauri-apps/api";
import Secret from "../interfaces/secret.interface";

export const useSecretStore = defineStore('secrets', () => {
    const secrets: Ref<Array<Secret>> = ref([]);
    const selectedSecret: Ref<Secret | undefined> = ref(undefined);
    const decryptedSecret: Ref<string | undefined> = ref(undefined);
    const passphrase: Ref<string | undefined> = ref(undefined);
    const isDecrypting: Ref<boolean> = ref(false);

    async function listItems() {
        const result: Array<Secret> = await invoke("list_items");
        secrets.value = result;
    }

    async function encryptItem(passphrase: string, category: string, key: string, data: string) {
        await invoke(
            "encrypt_item", 
            { 
                passphrase: passphrase, 
                category: category,
                key: key, 
                data: data 
            }
        );
    }
    
    async function decryptItem(passphrase: string, category: string, key: string) {
        isDecrypting.value = true;
        const result = await invoke(
            "decrypt_item", 
            { 
                passphrase: passphrase,
                category: category,
                key: key,
            }
        );
        isDecrypting.value = false;
        
        if (result) {
            decryptedSecret.value = result as string
        }
    }

    async function removeItem(passphrase: string, key: string) {
        await invoke(
            "remove_item", 
            { 
                passphrase: passphrase, 
                key: key 
            }
        );
    }

    return { secrets, selectedSecret, decryptedSecret, isDecrypting, passphrase, listItems, encryptItem, decryptItem, removeItem }
});