<template>
    <h1>{{ op }}</h1>
    <h2>DefaultGrammarSize</h2>
    <ul>
        <li v-for="(val, key) in content">{{ key }}: {{ val }}</li>
    </ul>
</template>

<script>
export default {
    props: ['op', 'content'],
}
</script>
