<template>
    <h1>scan</h1>
    <p>{{ names[content.symbol.n - 1] }}, {{ content.node }}</p>
</template>

<script>
export default {
    props: ['content', 'names']
}
</script>

<style>
</style>
