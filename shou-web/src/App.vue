<script setup lang="ts">
import { ref } from 'vue'
import { NSpace, NSlider } from 'naive-ui';
import axios from 'axios';

let v1 = ref(45);
let v2 = ref(45);
let v3 = ref(45);
let v4 = ref(45);
let v5 = ref(45);

</script>

<template>
  <n-space style="height: 300px; justify-content: center">
    <n-slider vertical :on-update:value="(value: number) => {
      v1 = value;
      console.log(value);
      axios.post('/api/set_angle', {
        num: 4,
        angle: value
      })
        .then(function (response) {
          console.log(response);
        })
        .catch(function (error) {
          console.log(error);
        });
    }" :step=5 :max=90 />
    <n-slider vertical :on-update:value="(value: number) => {
      v2 = value;
      console.log(value);
      axios.post('/api/set_angle', {
        num: 3,
        angle: value
      })
        .then(function (response) {
          console.log(response);
        })
        .catch(function (error) {
          console.log(error);
        });
    }" :step=5 :max=90 />
    <n-slider vertical :on-update:value="(value: number) => {
      v3 = value;
      console.log(value);
      axios.post('/api/set_angle', {
        num: 2,
        angle: value
      })
        .then(function (response) {
          console.log(response);
        })
        .catch(function (error) {
          console.log(error);
        });
    }" :step=5 :max=90 />
    <n-slider vertical :on-update:value="(value: number) => {
      v4 = value;
      console.log(value);
      axios.post('/api/set_angle', {
        num: 1,
        angle: value
      })
        .then(function (response) {
          console.log(response);
        })
        .catch(function (error) {
          console.log(error);
        });
    }" :step=5 :max=90 />
    <n-slider vertical :on-update:value="(value: number) => {
      v5 = value;
      console.log(value);
      axios.post('/api/set_angle', {
        num: 0,
        angle: 90 - value
      })
        .then(function (response) {
          console.log(response);
        })
        .catch(function (error) {
          console.log(error);
        });
    }" :step=5 :max=90 />
  </n-space>
</template>

<style scoped>
.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: filter 300ms;
}

.logo:hover {
  filter: drop-shadow(0 0 2em #646cffaa);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #42b883aa);
}
</style>
