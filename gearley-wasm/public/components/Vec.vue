<template>
    <Header :level="2" title="medial items">
        <ul>
            <template v-for="(item, index) of content">
                <li>
                    (origin: {{ item.origin }}, dot: {{ item.dot }})
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
