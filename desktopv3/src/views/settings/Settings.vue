<template>
  <div class="app-background">
    <div class="settings-widget">
      <div class="side-bar">
        <button>Profile</button>
				<button>Privacy</button>
				<button>Benjamin Netanyahu</button>
				<button>gilbert.</button>
				<button>settings5</button>
      </div>
			<div class="settings">
				<p>settings</p>
			</div>
    </div>
  </div>
</template>

<style scoped>
@import "/src/assets/styles/general.css";

.settings-widget {
  background-color: var(--theme-gray);
	border-radius: var(--theme-round-edges);
	display: flex;
	align-items: center;
}

.side-bar {
	border-right: solid 2px var(--theme-primary);
	display: flex;
	flex-direction: column;
	align-items: center;
	justify-content: space-between;
	margin: 10px;
	padding: 10px;
}

.side-bar button {
	font-weight: bold;
	cursor: pointer;
	width: 100%;
}

.settings {
	background-color: var(--theme-light-gray);
	border-radius: var(--theme-round-edges);
	display: flex;
	width: 100px;
	height: 200%;
}
</style>
