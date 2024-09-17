<template>
	<div class="todo-list">
		<h1>{{ title }}</h1>
		<input v-model="newTodo" @keyup.enter="addTodo" placeholder="Add a new todo" />
		<ul>
			<li v-for="(todo, index) in todos" :key="index">
				<span :class="{ completed: todo.completed }">
					{{ todo.text }}
				</span>
				<button @click="toggleTodo(index)">
					{{ todo.completed ? 'Undo' : 'Complete' }}
				</button>
				<button @click="removeTodo(index)">Remove</button>
			</li>
		</ul>
		<p>{{ remainingTodos }} todos left</p>
	</div>
</template>

<script>
export default {
	name: 'TodoList',
	data() {
		return {
			title: 'My Todo List',
			newTodo: '',
			todos: [
				{ text: 'Learn Vue.js', completed: false },
				{ text: 'Build a todo app', completed: false }
			]
		};
	},
	computed: {
		remainingTodos() {
			return this.todos.filter((todo) => !todo.completed).length;
		}
	},
	methods: {
		addTodo() {
			if (this.newTodo.trim().length === 0) {
				return;
			}
			this.todos.push({ text: this.newTodo, completed: false });
			this.newTodo = '';
		},
		toggleTodo(index) {
			this.todos[index].completed = !this.todos[index].completed;
		},
		removeTodo(index) {
			this.todos.splice(index, 1);
		}
	}
};
</script>

<style scoped>
.todo-list {
	font-family: Arial, sans-serif;
	max-width: 400px;
	margin: 0 auto;
}

ul {
	list-style-type: none;
	padding: 0;
}

li {
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 10px 0;
	border-bottom: 1px solid #eee;
}

.completed {
	text-decoration: line-through;
	color: #888;
}

input {
	width: 100%;
	padding: 5px;
	margin-bottom: 10px;
}

button {
	margin-left: 5px;
	padding: 2px 5px;
}
</style>
