<template>
    <span v-if="sym.n">{{ names[sym.n - 1] }} ({{ sym.n - 1 }})</span>
</template>

<script>
export default {
    props: ['sym', 'names'],
}
</script>
