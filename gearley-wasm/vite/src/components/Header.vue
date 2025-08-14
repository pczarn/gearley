<template>
  <div class="card">
    <header class="card-header" :class="`level-${level}`">
      <a class="button card-header-icon" @click="isCollapsed = !isCollapsed">
        <span v-if="!isCollapsed">&#x25BC;</span>
        <span v-if="isCollapsed">&#x25B6;</span>
      </a>
      <span class="card-header-title">{{title}}</span>
      <Button v-if="helpButton" label="Help" class="help-btn" @click="$emit('help')" />
      <Button v-if="printButton" label="Print" class="print-btn" @click="$emit('print')" />
    </header>
    <div class="card-content" :class="{'is-hidden':isCollapsed }">
      <slot v-if="!isCollapsed"></slot>
    </div>
    <footer class="card-footer" v-if="!isCollapsed">
      <slot name="footer"></slot>
    </footer>
  </div>
</template>

<script setup>
import Button from 'primevue/button'
import { onMounted, watch, ref } from 'vue';

Storage.prototype.setObject = function (key, value) {
  this.setItem(key, JSON.stringify(value));
};

Storage.prototype.getObject = function (key) {
  const value = this.getItem(key);
  return value && JSON.parse(value);
};

const props = defineProps({
    id: {
      type: String
    },
    title: {
      type: String,
      required: true
    },
    level: {
        type: Number,
        default: 1,
    },
    defaultCollapse: {
      type: Boolean,
      default: null
    },
    helpButton: {
      type: Boolean,
      default: false
    },
    printButton: {
      type: Boolean,
      default: false
    }
  });

const emit = defineEmits(['collapseChanged', 'print', 'help'])

function getCollapseState() {
  return localStorage.getObject("collapsibles") || {};
}

const isCollapsed = ref(props.defaultCollapse === null ? true : props.defaultCollapse)

onMounted(() => {
    if (props.defaultCollapse !== null && props.id) {
        saveCollapsedState(props.default)
    }
    if (props.id) {
      const state = getCollapseState();
      if (state) {
        isCollapsed.value = state[props.id];
      }
    }
})

function saveCollapsedState(value) {
  const state = getCollapseState();
  state[props.id] = value;
  localStorage.setObject("collapsibles", state);
}

watch(isCollapsed, (newValue) => {
  if (props.id) {
    saveCollapsedState(newValue);
  }
  emit("collapseChanged", newValue);
})
</script>

<style scoped>
.card-header {
  display: flex;
  align-items: center;
}

.help-btn {
  margin-left: auto;
}

.print-btn {
  margin-left: 15px;
}

.level-1 {
    font-size: 1.4em;
}

.level-2 {
    font-size: 1.1em;
}

.level-1 .card-header-icon {
    font-size: 0.9em;
    /* height: 1.1em;
    width: 1.1em; */
}

.level-2 .card-header-icon {
    font-size: 0.8em;
    /* height: 1em;
    width: 1em; */
}

.card-header-icon.button {
  margin-left: 0.5rem;
  margin-right: 0.5rem;
}
.card {
  margin: 0.5rem;
}

.card-content {
  margin-left: 1.25rem;
  margin-top: 0.75rem;
}
</style>
