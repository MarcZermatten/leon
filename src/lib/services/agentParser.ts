/**
 * Service pour parser l'output du terminal et détecter les agents actifs
 */

export interface Agent {
	id: string;
	name: string;
	type: string;
	status: 'running' | 'completed' | 'failed' | 'queued';
	startTime: number;
	endTime?: number;
	model: 'haiku' | 'sonnet' | 'opus';
	description?: string;
}

// Patterns pour détecter les agents dans l'output
const AGENT_PATTERNS = {
	// Task tool launch
	taskStart: /Using Task tool.*subagent_type[=:]?\s*['"]?(\w+[-\w]*)['"]?/i,
	taskStartAlt: /Task.*agent.*['"](\w+[-\w]*)['"]/i,

	// Agent descriptions in prompts
	agentDescription: /description[=:]?\s*["']([^"']+)["']/i,

	// Agent completion
	taskComplete: /Task.*completed|Agent.*finished|subagent.*done/i,
	taskFailed: /Task.*failed|Agent.*error|subagent.*failed/i,

	// Model detection
	modelHaiku: /model[=:]?\s*['"]?haiku['"]?/i,
	modelSonnet: /model[=:]?\s*['"]?sonnet['"]?/i,
	modelOpus: /model[=:]?\s*['"]?opus['"]?/i,

	// Agent ID in output
	agentId: /agentId:\s*([a-f0-9]+)/i,
};

// Map des agents connus et leurs modèles par défaut
const KNOWN_AGENTS: Record<string, { displayName: string; defaultModel: Agent['model'] }> = {
	'Explore': { displayName: 'Explorer', defaultModel: 'haiku' },
	'Plan': { displayName: 'Planificateur', defaultModel: 'sonnet' },
	'general-purpose': { displayName: 'Agent Général', defaultModel: 'sonnet' },
	'svelte-expert': { displayName: 'Expert Svelte', defaultModel: 'haiku' },
	'css-stylist': { displayName: 'Styliste CSS', defaultModel: 'haiku' },
	'component-builder': { displayName: 'Builder Composants', defaultModel: 'haiku' },
	'accessibility-checker': { displayName: 'Vérificateur A11y', defaultModel: 'haiku' },
	'tauri-expert': { displayName: 'Expert Tauri', defaultModel: 'haiku' },
	'rust-expert': { displayName: 'Expert Rust', defaultModel: 'haiku' },
	'typescript-fixer': { displayName: 'Correcteur TS', defaultModel: 'haiku' },
	'error-resolver': { displayName: 'Résolveur Erreurs', defaultModel: 'haiku' },
	'code-reviewer': { displayName: 'Reviewer Code', defaultModel: 'haiku' },
	'test-writer': { displayName: 'Écrivain Tests', defaultModel: 'haiku' },
	'bug-hunter': { displayName: 'Chasseur Bugs', defaultModel: 'sonnet' },
	'performance-optimizer': { displayName: 'Optimisateur Perf', defaultModel: 'sonnet' },
	'refactoring-assistant': { displayName: 'Assistant Refacto', defaultModel: 'haiku' },
	'git-assistant': { displayName: 'Assistant Git', defaultModel: 'haiku' },
	'dependency-analyzer': { displayName: 'Analyseur Deps', defaultModel: 'haiku' },
	'doc-writer': { displayName: 'Écrivain Doc', defaultModel: 'haiku' },
	'agent-manager': { displayName: 'Manager Agents', defaultModel: 'sonnet' },
	'spatial-analyst': { displayName: 'Analyste Spatial', defaultModel: 'sonnet' },
	'fme-specialist': { displayName: 'Spécialiste FME', defaultModel: 'sonnet' },
	'qgis-automation': { displayName: 'Automation QGIS', defaultModel: 'sonnet' },
};

let agentIdCounter = 0;

function generateAgentId(): string {
	return `agent_${Date.now()}_${++agentIdCounter}`;
}

export function detectModel(text: string): Agent['model'] {
	if (AGENT_PATTERNS.modelOpus.test(text)) return 'opus';
	if (AGENT_PATTERNS.modelSonnet.test(text)) return 'sonnet';
	if (AGENT_PATTERNS.modelHaiku.test(text)) return 'haiku';
	return 'haiku'; // default
}

export function parseAgentFromOutput(text: string): Partial<Agent> | null {
	// Try to detect a Task tool launch
	let match = text.match(AGENT_PATTERNS.taskStart);
	if (!match) {
		match = text.match(AGENT_PATTERNS.taskStartAlt);
	}

	if (match) {
		const agentType = match[1];
		const knownAgent = KNOWN_AGENTS[agentType];

		// Extract description if available
		const descMatch = text.match(AGENT_PATTERNS.agentDescription);
		const description = descMatch ? descMatch[1] : undefined;

		// Detect model
		const model = detectModel(text) || knownAgent?.defaultModel || 'haiku';

		return {
			id: generateAgentId(),
			name: knownAgent?.displayName || agentType,
			type: agentType,
			status: 'running',
			startTime: Date.now(),
			model,
			description: description || `Agent ${agentType}`
		};
	}

	return null;
}

export function detectAgentCompletion(text: string): { completed: boolean; failed: boolean; agentId?: string } {
	const idMatch = text.match(AGENT_PATTERNS.agentId);
	const agentId = idMatch ? idMatch[1] : undefined;

	if (AGENT_PATTERNS.taskFailed.test(text)) {
		return { completed: true, failed: true, agentId };
	}

	if (AGENT_PATTERNS.taskComplete.test(text)) {
		return { completed: true, failed: false, agentId };
	}

	return { completed: false, failed: false, agentId };
}

/**
 * Store pour gérer les agents actifs
 */
export class AgentStore {
	private agents: Map<string, Agent> = new Map();
	private listeners: Set<(agents: Agent[]) => void> = new Set();

	addAgent(agent: Agent): void {
		this.agents.set(agent.id, agent);
		this.notify();
	}

	updateAgent(id: string, updates: Partial<Agent>): void {
		const agent = this.agents.get(id);
		if (agent) {
			Object.assign(agent, updates);
			this.notify();
		}
	}

	completeAgent(id: string, failed: boolean = false): void {
		const agent = this.agents.get(id);
		if (agent) {
			agent.status = failed ? 'failed' : 'completed';
			agent.endTime = Date.now();
			this.notify();
		}
	}

	removeAgent(id: string): void {
		this.agents.delete(id);
		this.notify();
	}

	getAgents(): Agent[] {
		return Array.from(this.agents.values());
	}

	getRunningAgents(): Agent[] {
		return this.getAgents().filter(a => a.status === 'running');
	}

	clear(): void {
		this.agents.clear();
		this.notify();
	}

	subscribe(listener: (agents: Agent[]) => void): () => void {
		this.listeners.add(listener);
		listener(this.getAgents());
		return () => this.listeners.delete(listener);
	}

	private notify(): void {
		const agents = this.getAgents();
		this.listeners.forEach(l => l(agents));
	}

	/**
	 * Parse une ligne d'output et met à jour l'état
	 */
	processOutput(text: string): void {
		// Check for new agent
		const newAgent = parseAgentFromOutput(text);
		if (newAgent && newAgent.id) {
			this.addAgent(newAgent as Agent);
		}

		// Check for completion
		const completion = detectAgentCompletion(text);
		if (completion.completed) {
			// Complete the most recent running agent if no ID specified
			const runningAgents = this.getRunningAgents();
			if (runningAgents.length > 0) {
				const agentToComplete = runningAgents[runningAgents.length - 1];
				this.completeAgent(agentToComplete.id, completion.failed);
			}
		}
	}
}

// Singleton instance
export const agentStore = new AgentStore();
