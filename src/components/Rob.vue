<script setup lang="ts">
import * as types from "../utils/types";
import { watch } from "vue";

const props = defineProps({
  value: Array as () => Array<Array<types.RobEntry>>,
  height: String,
});

watch(() => props.value, () => {
  console.log(props.value);
})
</script>

<template>
  <v-col>
    <a>ROB</a>

    <v-table :height="props.height">
      <thead>
        <tr>
          <th colspan="4">bank 0</th>
          <th colspan="4">bank 1</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <th>valid</th>
          <th>phys_rd</th>
          <th>arch_rd</th>
          <th>ready</th>
          <th>valid</th>
          <th>phys_rd</th>
          <th>arch_rd</th>
          <th>ready</th>
        </tr>
        <tr v-for="entry in props.value">
          <td>{{ entry[0].entry_valid ? "✅" : "-" }}</td>
          <td>{{ entry[0].phys_rd }}</td>
          <td>{{ entry[0].arch_rd }}</td>
          <td>{{ entry[0].commit_ready ? "⭕" : "❌" }}</td>
          <td>{{ entry[1].entry_valid ? "✅" : "-" }}</td>
          <td>{{ entry[1].phys_rd }}</td>
          <td>{{ entry[1].arch_rd }}</td>
          <td>{{ entry[1].commit_ready ? "⭕" : "❌" }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-col>
</template>
