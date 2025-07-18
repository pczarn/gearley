<template>
  <div class="card">
    <header class="card-header" :class="`level-${level}`">
      <a class="button card-header-icon" @click="isCollapsed = !isCollapsed">
        <span v-if="!isCollapsed">&#x25BC;</span>
        <span v-if="isCollapsed">&#x25B6;</span>
      </a>
      <span class="card-header-title">{{title}}</span>
    </header>
    <div class="card-content" :class="{'is-hidden':isCollapsed }">
      <slot v-if="!isCollapsed"></slot>
    </div>
    <footer class="card-footer" v-if="!isCollapsed">
      <slot name="footer"></slot>
    </footer>
  </div>
</template>

<script>
Storage.prototype.setObject = function (key, value) {
  this.setItem(key, JSON.stringify(value));
};

Storage.prototype.getObject = function (key) {
  const value = this.getItem(key);
  return value && JSON.parse(value);
};

export default {
  mounted() {
    if (this.defaultCollapse !== null && this.id) {
        this.saveCollapsedState(this.default)
    }
    if (this.id) {
      const state = this.getCollapseState();
      if (state) {
        this.isCollapsed = state[this.id];
      }
    }
  },
  props: {
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
    }
  },
  data() {
    return {
      isCollapsed: this.defaultCollapse === null ? true : this.defaultCollapse
    };
  },
  methods: {
    getCollapseState() {
      return localStorage.getObject("collapsibles") || {};
    },
    saveCollapsedState(value) {
      const state = this.getCollapseState();
      state[this.id] = value;
      localStorage.setObject("collapsibles", state);
    }
  },
  watch: {
    isCollapsed(newValue) {
      if (this.id) {
        this.saveCollapsedState(newValue);
      }
      this.$emit("collapseChanged", newValue);
    }
  }
};
</script>

<style scoped>
.card-header {
  display: flex;
  align-items: center;
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
