<template>
    <ul>
        <li v-for="(val, key) in content">{{ key }}: {{ val }}</li>
    </ul>
</template>

<script>
export default {
    props: ['op', 'content', 'names']
}
</script>
