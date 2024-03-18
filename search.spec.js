//const shallowMount = require('@vue/test-utils');
//const Mods = require('@/pages/instance/Mods.vue');

import { shallowMount } from '@vue/test-utils';
import { ref } from 'vue';
import { onSearchChange } from '../browse_test.js';

test('the data is peanut butter', async () => {
  const query = ref('')
  query.value = ''
  await onSearchChange(query);
});