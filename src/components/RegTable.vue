<script setup lang="ts">
import { ref, watch } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  name: String,
  keys: Array as () => Array<string>,
  value: Array as () => Array<string>,
});

const table = ref([ ] as Array<[string, string]>)
const len = props.keys?.length ?? 32;
console.log(len);
for (let i = 0; i < len; i++) {
  let values = props.value ?? [];
  let key = props.keys ?? [];
  table.value.push([
    key[i] ?? "0",
    values[i] ?? "0",
  ])
}

watch(() => props.value, (newVal) => {
  if (!newVal) {
    console.log("newVal is null");
  }
  let values = newVal ?? [];
  for (let i = 0; i < table.value.length; i++) {
    table.value[i][1] = values[i] ?? "0";
  }
})
</script>

<template>
  <v-table>
    <thead>
      <tr>
        <th colspan="2">{{ props.name }}</th>
      </tr>
    </thead>
    <tbody>
      <tr>
        <th>Register</th>
        <th>Value</th>
      </tr>
      <tr v-for="row in table">
        <td>{{ row[0] }}</td>
        <td>{{ row[1] }}</td>
      </tr>
    </tbody>
  </v-table>
</template>

