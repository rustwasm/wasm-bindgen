interface mixin Animatable {
    Animation           animate(object? keyframes,
                                optional (unrestricted double or KeyframeAnimationOptions) options = {});
    sequence<Animation> getAnimations(optional GetAnimationsOptions options = {});
};

dictionary KeyframeAnimationOptions : KeyframeEffectOptions {
    DOMString id = "";
    AnimationTimeline? timeline;
};

dictionary GetAnimationsOptions {
    boolean subtree = false;
};
