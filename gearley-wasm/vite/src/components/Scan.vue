<template>
    <p>{{ names[content.symbol.n - 1] }}, {{ content.node }}</p>
</template>

<script>
export default {
    props: ['content', 'names', 'op']
}
</script>

<style>
</style>
