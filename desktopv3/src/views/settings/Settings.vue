<script setup>
import { ref } from "vue";

const selectedSetting = ref("profile");
</script>

<template>
  <div class="app-background">
    <div class="settings-widget">
			<h1>Settings</h1>
			<div class="settings-container">
				<div class="side-bar">
					<label class="side-bar-option">
						<input
							type="radio"
							name="settings-panel"
							value="profile"
							v-model="selectedSetting"
						/>
						<span class="side-bar-label">Profile</span>
					</label>
					<label class="side-bar-option">
						<input
							type="radio"
							name="settings-panel"
							value="privacy"
							v-model="selectedSetting"
						/>
						<span class="side-bar-label">Privacy</span>
					</label>
					<label class="side-bar-option">
						<input
							type="radio"
							name="settings-panel"
							value="benjamin"
							v-model="selectedSetting"
						/>
						<span class="side-bar-label">Benjamin Netanyahu</span>
					</label>
					<label class="side-bar-option">
						<input
							type="radio"
							name="settings-panel"
							value="gilbert"
							v-model="selectedSetting"
						/>
						<span class="side-bar-label">gilbert.</span>
					</label>
					<label class="side-bar-option">
						<input
							type="radio"
							name="settings-panel"
							value="settings5"
							v-model="selectedSetting"
						/>
						<span class="side-bar-label">settings5</span>
					</label>
				</div>
				<template v-if="selectedSetting === 'benjamin'">
					<div class="main-settings">
						<h2>benjamins settings</h2>
						<div class="settings">
							<div>
								<input type="checkbox" name="setting1" /><label for="setting1">Setting1</label>
							</div>
							<div>
								<input type="checkbox" name="setting2" /><label for="setting2">Setting2</label>
							</div>
							<div>
								<input type="checkbox" name="setting3" /><label for="setting3">Setting3</label>
							</div>
							<div>
								<input type="checkbox" name="setting4" /><label for="setting4">Setting4</label>
							</div>
						</div>
					</div>
				</template>
				<template v-else-if="selectedSetting === 'profile'">
					<p>profile settings</p>
				</template>
			</div>
		</div>
  </div>
</template>

<style scoped>
@import "../../assets/styles/general.css";
@import "../../assets/styles/root.css";
@import "../../assets/styles/utils/animations.css";

.settings-widget {
  background-color: var(--theme-gray);
  border-radius: var(--theme-round-edges);
  display: flex;
	flex-direction: column;
  align-items: center;
	animation: fadeIn 0.5s ease-in-out
}

.settings-container {
	display: flex;
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

.side-bar-option {
  display: flex;
  align-items: center;
  width: 100%;
  margin: 2px 0;
}

.side-bar-option input[type="radio"] {
  position: absolute;
  opacity: 0;
  pointer-events: none;
}

.side-bar-label {
  font-weight: bold;
  cursor: pointer;
  width: 100%;
  padding: 4px 8px;
  color: var(--theme-white);
  background-color: var(--theme-gray);
  border: 3px solid var(--theme-primary);
  border-radius: var(--theme-round-edges);
  text-align: center;
}

.side-bar-option input[type="radio"]:checked + .side-bar-label {
  background-color: var(--theme-primary);
  color: var(--theme-black);
}

.main-settings {
  padding: 10px;
  margin: 10px;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
	animation: slide 0.5s ease-in-out;
}

.settings label {
  color: var(--theme-white);
}

.settings div {
  display: flex;
  padding-bottom: 5px;
}

.settings input[type="checkbox"] {
  margin-right: 10px;
}

@keyframes slide {
	from {
		opacity: 0;
		transform: translateX(-40px);
	}

	to {
		opacity: 1;
		transform: translateX(0px);
	}
}
</style>
