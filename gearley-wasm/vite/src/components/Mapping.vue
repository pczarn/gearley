<template>
    <h1>{{ op }}</h1>
    <h2>Mapping</h2>
    <ul>
        <ol start="0">
            <li v-for="sym_with_name in content">{{ sym_with_name.name ? sym_with_name.name.name : 'None' }} ({{ sym_with_name.sym.n }})</li>
        </ol>
    </ul>
</template>

<script>
export default {
    props: ['op', 'content', 'names']
}
</script>
