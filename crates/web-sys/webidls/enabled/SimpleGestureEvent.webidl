/* -*- Mode: IDL; tab-width: 2; indent-tabs-mode: nil; c-basic-offset: 2 -*- */
/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this file,
 * You can obtain one at http://mozilla.org/MPL/2.0/.
 */

/**
 * The SimpleGestureEvent interface is the datatype for all
 * Mozilla-specific simple gesture events in the Document Object Model.
 *
 * The following events are generated:
 *
 * MozSwipeGestureMayStart - Generated when the user starts a horizontal
 * swipe across the input device, but before we know whether the user
 * is actually scrolling past a scroll edge.
 * This event asks two questions:  Should a swipe really be started, and
 * in which directions should the user be able to swipe?  The first
 * question is answered by event listeners by calling or not calling
 * preventDefault() on the event.  Since a swipe swallows all scroll
 * events, the default action of the swipe start event is *not* to
 * start a swipe. Call preventDefault() if you want a swipe to be
 * started. Doing so won't necessarily result in a swipe being started,
 * it only communicates an intention. Once Gecko determines whether a
 * swipe should actually be started, it will send a MozSwipeGestureStart
 * event.
 * The second question (swipe-able directions) is answered in the
 * allowedDirections field.
 *
 * MozSwipeGestureStart - This event signals the start of a swipe.
 * It guarantees a future MozSwipeGestureEnd event that will signal
 * the end of a swipe animation.
 *
 * MozSwipeGestureUpdate - Generated periodically while the user is
 * continuing a horizontal swipe gesture.  The "delta" value represents
 * the current absolute gesture amount.  This event may even be sent
 * after a MozSwipeGesture event fired in order to allow for fluid
 * completion of a swipe animation.  The direction value is meaningless
 * on swipe update events.
 *
 * MozSwipeGestureEnd - Generated when the swipe animation is completed.
 *
 * MozSwipeGesture - Generated when the user releases a swipe across
 * across the input device.  This event signals that the actual swipe
 * operation is complete, even though the animation might not be finished
 * yet.  This event can be sent without accompanying start / update / end
 * events, and it can also be handled on its own if the consumer doesn't
 * want to handle swipe animation events.
 * Only the direction value has any significance, the delta value is
 * meaningless.
 *
 * MozMagnifyGestureStart - Generated when the user begins the magnify
 * ("pinch") gesture.  The "delta" value represents the initial
 * movement.
 *
 * MozMagnifyGestureUpdate - Generated periodically while the user is
 * continuing the magnify ("pinch") gesture.  The "delta" value
 * represents the movement since the last MozMagnifyGestureStart or
 * MozMagnifyGestureUpdate event.
 *
 * MozMagnifyGesture - Generated when the user has completed the
 * magnify ("pinch") gesture.  If you only want to receive a single
 * event when the magnify gesture is complete, you only need to hook
 * this event and can safely ignore the MozMagnifyGestureStart and the
 * MozMagnifyGestureUpdate events. The "delta" value is the cumulative
 * amount represented by the user's gesture.
 *
 * MozRotateGestureStart - Generated when the user begins the rotation
 * gesture.  The "delta" value represents the initial rotation.
 *
 * MozRotateGestureUpdate - Generated periodically while the user is
 * continuing the rotation gesture.  The "delta" value represents the
 * rotation since the last MozRotateGestureStart or
 * MozRotateGestureUpdate event.
 *
 * MozRotateGesture - Generated when the user has completed the
 * rotation gesture.  If you only want to receive a single event when
 * the rotation gesture is complete, you only need to hook this event
 * and can safely ignore the MozRotateGestureStart and the
 * MozRotateGestureUpdate events.  The "delta" value is the cumulative
 * amount of rotation represented by the user's gesture.
 *
 * MozTapGesture - Generated when the user executes a two finger
 * tap gesture on the input device. Client coordinates contain the
 * center point of the tap.
 * (XXX On OS X, only Lion (10.7) and up)
 *
 * MozPressTapGesture - Generated when the user executes a press
 * and tap two finger gesture (first finger down, second finger down,
 * second finger up, first finger up) on the input device.
 * Client coordinates contain the center pivot point of the action.
 * (XXX Not implemented on Mac)
 *
 * MozEdgeUIGesture - Generated when the user swipes the display to
 * invoke edge ui.
 * (XXX Win8 only)
 *
 * Default behavior:
 *
 * Some operating systems support default behaviors for gesture events
 * when they are not handled by the application. Consumers should
 * use event.preventDefault() to prevent default behavior when
 * consuming events.
 */

[Func="IsChromeOrXBL"]
interface SimpleGestureEvent : MouseEvent
{
  /* Swipe direction constants */
  const unsigned long DIRECTION_UP = 1;
  const unsigned long DIRECTION_DOWN = 2;
  const unsigned long DIRECTION_LEFT = 4;
  const unsigned long DIRECTION_RIGHT = 8;

  /* Rotational direction constants */
  const unsigned long ROTATION_COUNTERCLOCKWISE = 1;
  const unsigned long ROTATION_CLOCKWISE = 2;

  /* Read-write value for swipe events.
   *
   * Reports the directions that can be swiped to; multiple directions
   * should be OR'ed together.
   *
   * The allowedDirections field is designed to be set on SwipeGestureMayStart
   * events by event listeners.  Its value after event dispatch determines
   * the behavior of the swipe animation that might be about to begin.
   * Specifically, if the user swipes in a direction that can't be swiped
   * to, the animation will have a bounce effect.
   * Future SwipeGestureUpdate, SwipeGesture and SwipeGestureEnd events
   * will carry the allowDirections value that was set on the SwipeMayStart
   * event.  Changing this field on non-SwipeGestureMayStart events doesn't
   * have any effect.
   */
  attribute unsigned long allowedDirections;

  /* Direction of a gesture. Diagonals are indicated by OR'ing the
   * applicable constants together.
   *
   * Swipes gestures may occur in any direction.
   *
   * Magnify gestures do not have a direction.
   *
   * Rotation gestures will be either ROTATION_COUNTERCLOCKWISE or
   * ROTATION_CLOCKWISE.
   */
  readonly attribute unsigned long direction;

  /* Delta value for magnify, rotate and swipe gestures.
   *
   * For rotation, the value is in degrees and is positive for
   * clockwise rotation and negative for counterclockwise
   * rotation.
   *
   * For magnification, the value will be positive for a "zoom in"
   * (i.e, increased magnification) and negative for a "zoom out"
   * (i.e., decreased magnification).  The particular units
   * represented by the "delta" are currently implementation specific.
   *
   * XXX - The units for measuring magnification are currently
   * unspecified because the units used by Mac OS X are currently
   * undocumented.  The values are typically in the range of 0.0 to
   * 100.0, but it is only safe currently to rely on the delta being
   * positive or negative.
   *
   * For swipe start, update and end events, the value is a fraction
   * of one "page".  If the resulting swipe will have DIRECTION_LEFT, the
   * delta value will be positive; for DIRECTION_RIGHT, delta is negative.
   * If this seems backwards to you, look at it this way:  If the current
   * page is pushed to the right during the animation (positive delta),
   * the page left to the current page will be visible after the swipe
   * (DIRECTION_LEFT).
   *
   * Units on Windows represent the difference between the initial
   * and current/final width between the two touch points on the input
   * device and are measured in pixels.
   */
  readonly attribute double delta;

  /* Click count value for taps. */
  readonly attribute unsigned long clickCount;

  void initSimpleGestureEvent(DOMString typeArg,
                              optional boolean canBubbleArg = false,
                              optional boolean cancelableArg = false,
                              optional Window? viewArg = null,
                              optional long detailArg = 0,
                              optional long screenXArg = 0,
                              optional long screenYArg = 0,
                              optional long clientXArg = 0,
                              optional long clientYArg = 0,
                              optional boolean ctrlKeyArg = false,
                              optional boolean altKeyArg = false,
                              optional boolean shiftKeyArg = false,
                              optional boolean metaKeyArg = false,
                              optional short buttonArg = 0,
                              optional EventTarget? relatedTargetArg = null,
                              optional unsigned long allowedDirectionsArg = 0,
                              optional unsigned long directionArg = 0,
                              optional double deltaArg = 0,
                              optional unsigned long clickCount = 0);
};
