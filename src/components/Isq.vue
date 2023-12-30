<script setup lang="ts">
import * as types from "../utils/types";
import { watch } from "vue";

const props = defineProps({
  value: Array as () => Array<types.IsqEntry>,
  height: String,
});

watch(() => props.value, () => {
  console.log(props.value);
})
</script>

<template>
  <v-col>
    <a>Issue Queue</a>

    <v-table :height="props.height">
      <tbody>
        <tr>
          <th>valid</th>
          <!--<th>alu_cmd</th>-->
          <th>op1_valid</th>
          <th>op1_data</th>
          <th>op2_valid</th>
          <!--<th>op2_type</th>-->
          <th>op2_data</th>
          <th>phys_rd</th>
          <th>bank</th>
          <th>addr</th>
        </tr>
        <tr v-for="entry in props.value">
          <td>{{ entry.entry_valid ? "✅" : "-" }}</td>
          <!--<td>{{ entry.alu_cmd }}</td>-->
          <td>{{ entry.op1_valid ? "✅" : "-" }}</td>
          <td>{{ entry.op1_data }}</td>
          <td>{{ entry.op2_valid ? "✅" : "-" }}</td>
          <!--<td>{{ entry.op2_type }}</td>-->
          <td>{{ entry.op2_data }}</td>
          <td>{{ entry.phys_rd }}</td>
          <td>{{ entry.bank_addr }}</td>
          <td>{{ entry.rob_addr }}</td>
        </tr>
      </tbody>
    </v-table>
  </v-col>
</template>

