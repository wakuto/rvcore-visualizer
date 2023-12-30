<script setup lang="ts">
import { ref, watch } from "vue";
// import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps({
  name: String,
  keys: Array as () => Array<string>,
  value: Array as () => Array<string>,
  phys_regs: Array as () => Array<number>,
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
  <v-col>
    <a>{{  props.name }}</a>

    <v-table>
      <tbody>
        <tr>
          <th>Register</th>
          <td v-for="col in table">{{  col[0] }}</td>
        </tr>
        <tr>
          <th>Value</th>
          <td v-for="col in table"> {{ col[1] }}</td>
        </tr>
        <tr v-if="props.phys_regs">
          <th>PhysRegs Value</th>
          <td v-for="col in table"> {{ props.phys_regs[Number(col[1])] }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-col>
</template>

