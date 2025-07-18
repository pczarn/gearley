<template>
    <Header :level="2" title="predicted symbols">
        <ul>
            <template v-for="(ch, index) of content.row.trim()">
                <li v-if="ch === '1'">
                    {{ names[index] }}
                </li>
            </template>
        </ul>
    </Header>
</template>

<script>
import Header from './Header.vue'

export default {
    props: ['op', 'content', 'names'],
    components: {
        Header
    }
}
</script>

<style>
</style>
