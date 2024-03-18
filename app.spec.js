//const shallowMount = require('@vue/test-utils');
//const Mods = require('@/pages/instance/Mods.vue');

import { shallowMount } from '@vue/test-utils';
import Mods from '@/pages/instance/Mods.vue';

describe('Mods.vue', () => {
  it('checks the addition logic', () => {
    const wrapper = shallowMount(Mods);

    const componentText = wrapper.text();
    expect(componentText).toContain('Actions');
  });
});