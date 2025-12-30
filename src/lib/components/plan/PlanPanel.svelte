<script lang="ts">
	import { CheckCircle2, Circle, ListChecks, GitBranch, ChevronDown, ChevronRight } from 'lucide-svelte';

	interface PlanStep {
		id: string;
		text: string;
		completed: boolean;
		children?: PlanStep[];
	}

	interface Plan {
		title: string;
		steps: PlanStep[];
		mermaid?: string;
	}

	let {
		isVisible = false,
		onClose = () => {}
	} = $props<{
		isVisible: boolean;
		onClose: () => void;
	}>();

	let plan = $state<Plan | null>(null);
	let expandedSteps = $state<Set<string>>(new Set());
	let mermaidRendered = $state<string | null>(null);

	// Parser le texte du terminal pour extraire un plan
	export function parsePlanFromText(text: string): boolean {
		// Chercher des patterns de plan
		const planPatterns = [
			/##?\s*Plan[:\s]*\n([\s\S]*?)(?=\n##|\n---|\Z)/i,
			/(?:Here'?s? (?:the|my) plan|I'?ll|Let me)[:\s]*\n([\s\S]*?)(?=\n##|\n---|\Z)/i,
			/(?:Étapes?|Steps?)[:\s]*\n([\s\S]*?)(?=\n##|\n---|\Z)/i
		];

		for (const pattern of planPatterns) {
			const match = text.match(pattern);
			if (match) {
				const planText = match[1] || match[0];
				const extractedPlan = extractPlan(planText);
				if (extractedPlan && extractedPlan.steps.length > 0) {
					plan = extractedPlan;
					return true;
				}
			}
		}

		// Chercher des listes numérotées ou à puces
		const listMatch = text.match(/(?:^|\n)((?:\s*[-*\d.]+\s+.+\n?)+)/m);
		if (listMatch) {
			const extractedPlan = extractPlan(listMatch[1]);
			if (extractedPlan && extractedPlan.steps.length >= 2) {
				plan = extractedPlan;
				return true;
			}
		}

		return false;
	}

	function extractPlan(text: string): Plan {
		const lines = text.split('\n').filter(l => l.trim());
		const steps: PlanStep[] = [];
		let title = 'Plan';

		// Extraire le titre si présent
		const titleMatch = text.match(/^#*\s*(.+?)(?:\n|$)/);
		if (titleMatch && !titleMatch[1].match(/^[-*\d]/)) {
			title = titleMatch[1].trim();
		}

		// Parser les étapes
		for (const line of lines) {
			const stepMatch = line.match(/^\s*([-*]|\d+[.):]?)\s*\[?([ xX])?\]?\s*(.+)/);
			if (stepMatch) {
				const completed = stepMatch[2]?.toLowerCase() === 'x';
				const text = stepMatch[3].trim();
				steps.push({
					id: crypto.randomUUID(),
					text,
					completed
				});
			}
		}

		// Chercher un diagramme Mermaid
		const mermaidMatch = text.match(/```mermaid\n([\s\S]*?)```/);
		const mermaid = mermaidMatch ? mermaidMatch[1].trim() : undefined;

		return { title, steps, mermaid };
	}

	function toggleStep(stepId: string) {
		if (!plan) return;
		plan = {
			...plan,
			steps: plan.steps.map(s =>
				s.id === stepId ? { ...s, completed: !s.completed } : s
			)
		};
	}

	function toggleExpand(stepId: string) {
		const newSet = new Set(expandedSteps);
		if (newSet.has(stepId)) {
			newSet.delete(stepId);
		} else {
			newSet.add(stepId);
		}
		expandedSteps = newSet;
	}

	// Calculer la progression
	let progress = $derived(() => {
		if (!plan || plan.steps.length === 0) return 0;
		const completed = plan.steps.filter(s => s.completed).length;
		return Math.round((completed / plan.steps.length) * 100);
	});

	// Méthode publique pour définir un plan
	export function setPlan(newPlan: Plan) {
		plan = newPlan;
	}

	// Méthode publique pour marquer une étape comme complétée
	export function completeStep(index: number) {
		if (!plan || index >= plan.steps.length) return;
		plan = {
			...plan,
			steps: plan.steps.map((s, i) =>
				i === index ? { ...s, completed: true } : s
			)
		};
	}

	// Méthode publique pour reset
	export function reset() {
		plan = null;
		expandedSteps = new Set();
	}
</script>

{#if isVisible && plan}
	<div class="plan-panel">
		<div class="plan-header">
			<div class="plan-title">
				<ListChecks size={18} />
				<h3>{plan.title}</h3>
			</div>
			<div class="plan-progress">
				<div class="progress-bar">
					<div class="progress-fill" style="width: {progress()}%"></div>
				</div>
				<span class="progress-text">{progress()}%</span>
			</div>
			<button class="close-btn" onclick={onClose}>×</button>
		</div>

		<div class="plan-content">
			<div class="steps-list">
				{#each plan.steps as step, index (step.id)}
					<div class="step" class:completed={step.completed}>
						<button class="step-toggle" onclick={() => toggleStep(step.id)}>
							{#if step.completed}
								<CheckCircle2 size={18} class="check-icon" />
							{:else}
								<Circle size={18} />
							{/if}
						</button>
						<span class="step-number">{index + 1}.</span>
						<span class="step-text">{step.text}</span>
					</div>
				{/each}
			</div>

			{#if plan.mermaid}
				<div class="mermaid-section">
					<div class="mermaid-header">
						<GitBranch size={16} />
						<span>Diagramme</span>
					</div>
					<div class="mermaid-code">
						<pre>{plan.mermaid}</pre>
					</div>
				</div>
			{/if}
		</div>
	</div>
{/if}

<style>
	.plan-panel {
		display: flex;
		flex-direction: column;
		height: 100%;
		background: var(--color-bg-secondary);
		border-left: 1px solid var(--color-border);
	}

	.plan-header {
		display: flex;
		align-items: center;
		gap: 1rem;
		padding: 0.75rem 1rem;
		border-bottom: 1px solid var(--color-border);
		background: var(--color-bg-tertiary);
	}

	.plan-title {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		flex: 1;
		color: var(--color-lion-400);
	}

	.plan-title h3 {
		margin: 0;
		font-size: 0.9rem;
		font-weight: 600;
	}

	.plan-progress {
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.progress-bar {
		width: 80px;
		height: 6px;
		background: var(--color-bg-primary);
		border-radius: 3px;
		overflow: hidden;
	}

	.progress-fill {
		height: 100%;
		background: var(--color-lion-500);
		border-radius: 3px;
		transition: width 0.3s ease;
	}

	.progress-text {
		font-size: 0.75rem;
		font-family: 'JetBrains Mono', monospace;
		color: var(--color-text-muted);
		min-width: 35px;
	}

	.close-btn {
		background: none;
		border: none;
		color: var(--color-text-muted);
		font-size: 1.25rem;
		cursor: pointer;
		padding: 0.25rem;
		line-height: 1;
	}

	.close-btn:hover {
		color: var(--color-text-primary);
	}

	.plan-content {
		flex: 1;
		overflow-y: auto;
		padding: 1rem;
	}

	.steps-list {
		display: flex;
		flex-direction: column;
		gap: 0.5rem;
	}

	.step {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.5rem;
		border-radius: 6px;
		transition: background 0.15s ease;
	}

	.step:hover {
		background: var(--color-bg-hover);
	}

	.step.completed {
		opacity: 0.6;
	}

	.step.completed .step-text {
		text-decoration: line-through;
	}

	.step-toggle {
		background: none;
		border: none;
		color: var(--color-text-muted);
		cursor: pointer;
		padding: 0;
		display: flex;
		align-items: center;
	}

	.step-toggle:hover {
		color: var(--color-lion-400);
	}

	.step.completed .step-toggle {
		color: var(--color-success, #69db7c);
	}

	.step-number {
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--color-text-muted);
		min-width: 20px;
	}

	.step-text {
		font-size: 0.875rem;
		color: var(--color-text-primary);
		line-height: 1.4;
	}

	.mermaid-section {
		margin-top: 1.5rem;
		padding-top: 1rem;
		border-top: 1px solid var(--color-border);
	}

	.mermaid-header {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		font-size: 0.8rem;
		font-weight: 600;
		color: var(--color-text-secondary);
		margin-bottom: 0.75rem;
	}

	.mermaid-code {
		background: var(--color-bg-primary);
		border-radius: 6px;
		padding: 0.75rem;
		overflow-x: auto;
	}

	.mermaid-code pre {
		margin: 0;
		font-size: 0.75rem;
		font-family: 'JetBrains Mono', monospace;
		color: var(--color-text-secondary);
	}
</style>
