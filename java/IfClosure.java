public class IfClosure implements NixObject {

	@Override
	public NixObject call(NixObject condition) {
		return trueCase -> falseCase -> (force) -> {
			NixBoolean evaluatedCondition = (NixBoolean) condition.call(null);
			if (evaluatedCondition.value) {
				return trueCase.call(null);
			} else {
				return falseCase.call(null);
			}
		};
	}
}
