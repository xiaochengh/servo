/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use dom_struct::dom_struct;
use html5ever::{LocalName, Prefix};
use js::rust::HandleObject;

use crate::dom::bindings::codegen::Bindings::HTMLParagraphElementBinding::HTMLParagraphElementMethods;
use crate::dom::bindings::root::DomRoot;
use crate::dom::bindings::str::DOMString;
use crate::dom::document::Document;
use crate::dom::htmlelement::HTMLElement;
use crate::dom::node::Node;
use crate::script_runtime::CanGc;

#[dom_struct]
pub(crate) struct HTMLParagraphElement {
    htmlelement: HTMLElement,
}

impl HTMLParagraphElement {
    fn new_inherited(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
    ) -> HTMLParagraphElement {
        HTMLParagraphElement {
            htmlelement: HTMLElement::new_inherited(local_name, prefix, document),
        }
    }

    #[cfg_attr(crown, allow(crown::unrooted_must_root))]
    pub(crate) fn new(
        local_name: LocalName,
        prefix: Option<Prefix>,
        document: &Document,
        proto: Option<HandleObject>,
        can_gc: CanGc,
    ) -> DomRoot<HTMLParagraphElement> {
        Node::reflect_node_with_proto(
            Box::new(HTMLParagraphElement::new_inherited(
                local_name, prefix, document,
            )),
            document,
            proto,
            can_gc,
        )
    }
}

impl HTMLParagraphElementMethods<crate::DomTypeHolder> for HTMLParagraphElement {
    // https://html.spec.whatwg.org/multipage/#dom-p-align
    make_getter!(Align, "align");

    // https://html.spec.whatwg.org/multipage/#dom-p-align
    make_setter!(SetAlign, "align");
}
