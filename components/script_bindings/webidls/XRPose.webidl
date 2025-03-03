/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

// skip-unless CARGO_FEATURE_WEBXR

// https://immersive-web.github.io/webxr/#xrpose-interface

[SecureContext, Exposed=Window, Pref="dom_webxr_enabled"]
interface XRPose {
  [SameObject] readonly attribute XRRigidTransform transform;
  [SameObject] readonly attribute DOMPointReadOnly? linearVelocity;
  [SameObject] readonly attribute DOMPointReadOnly? angularVelocity;

  readonly attribute boolean emulatedPosition;
};
