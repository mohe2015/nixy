public class IfClosure implements NixObject {

	@Override
	public NixObject call(NixObject condition) {
		return new NixObject() {
			@Override
			public NixObject call(NixObject trueCase) {
				return new NixObject() {
					@Override
					public NixObject call(NixObject falseCase) {
						NixBoolean evaluatedCondition = (NixBoolean) condition.call(null);
						if (evaluatedCondition.value) {
							return trueCase.call(null);
						} else {
							return falseCase.call(null);
						}
					}
				};
			}
		};
	}
}
