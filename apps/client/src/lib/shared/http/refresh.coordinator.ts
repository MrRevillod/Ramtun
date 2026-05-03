class RefreshCoordinator {
	#inFlight: Promise<string | null> | null = null

	public run(task: () => Promise<string | null>): Promise<string | null> {
		if (!this.#inFlight) {
			this.#inFlight = task().finally(() => {
				this.#inFlight = null
			})
		}

		return this.#inFlight
	}
}

export const refreshCoordinator = new RefreshCoordinator()
