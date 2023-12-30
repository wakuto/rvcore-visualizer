<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import RegTable from "./components/RegTable.vue";
import Rob from "./components/Rob.vue";
import Isq from "./components/Isq.vue";
import { reactive, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event";

// risc-v
const reg_table = [
    "x0(zero)", "x1(ra)", "x2(sp)", "x3(gp)",
    "x4(tp)", "x5(t0)", "x6(t1)", "x7(t2)",
    "x8(s0/fp)", "x9(s1)", "x10(a0)", "x11(a1)",
    "x12(a2)", "x13(a3)", "x14(a4)", "x15(a5)",
    "x16(a6)", "x17(a7)", "x18(s2)", "x19(s3)",
    "x20(s4)", "x21(s5)", "x22(s6)", "x23(s7)",
    "x24(s8)", "x25(s9)", "x26(s10)", "x27(s11)",
    "x28(t3)", "x29(t4)", "x30(t5)", "x31(t6)"
];

let rename_map_table = reactive({ value: [] })
let commit_map_table = reactive({ value: [] })
let physical_reg_file = reactive({ value: [] })
let rob = reactive({ value: [] })
let isq = reactive({ value: [] })

let signals: Array<any> = []
let clock_num = reactive({value: 0});

listen<string>('vcd_file_selected', (event) => {
  for (let i = 0; i < reg_table.length; i++) {
    rename_map_table.value = [];
    commit_map_table.value = [];
    physical_reg_file.value = [];
    isq.value = [];
    rob.value = [];
  }
  clock_num.value = 0;
  signals = JSON.parse(event.payload);
});

async function openVcdFile() {
  await invoke("open_vcd_file");
}

watch(() => clock_num.value, () => {
  rename_map_table.value = signals[clock_num.value]["regfiles"]["rename_map_table"] || [];  
  commit_map_table.value = signals[clock_num.value]["regfiles"]["commit_map_table"] || [];
  physical_reg_file.value = signals[clock_num.value]["regfiles"]["physical_regfile"] || [];
  rob.value = signals[clock_num.value]["rob"] || [];
  isq.value = signals[clock_num.value]["isq"] || [];
  if (physical_reg_file.value.length == 0) {
    console.log("physical_reg_file is empty");
    console.log(signals[clock_num.value])
  }
})

</script>

<template>
  <div class="container">
    <!-- file open button -->
    <v-container class="controller_container">
      <v-row> <v-col> <v-btn @click="openVcdFile">Open VCD File</v-btn> </v-col> </v-row>
      <v-row>
        <v-col>
          <v-btn @click="clock_num.value--">Prev Clock</v-btn>
        </v-col>
        <v-col>
          <v-btn @click="clock_num.value = 0">Reset</v-btn>
        </v-col>
        <v-col>
          <v-btn @click="clock_num.value++">Next Clock</v-btn>
        </v-col>
      </v-row>
    </v-container>
    <a>Clock: {{ clock_num.value }}</a><br>
    <v-row>
      <v-col cols="6"> <Rob height="20vh" :value="rob.value"/> </v-col>      
      <v-col cols="6"> <Isq height="20vh" :value="isq.value"/> </v-col>
    </v-row>
    <div class="reg_container">
      <v-container>
        <v-row>
          <v-col> <RegTable name="rename_map_table" :keys="reg_table" :value="rename_map_table.value" /> </v-col>
          <v-col> <RegTable name="commit_map_table" :keys="reg_table" :value="commit_map_table.value" :phys_regs="physical_reg_file.value"/> </v-col>
          <v-col> <RegTable name="physical_reg_file" :keys="Array.from({length: 64}, (_, k) => k.toString())" :value="physical_reg_file.value"/> </v-col>
        </v-row>
      </v-container>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
