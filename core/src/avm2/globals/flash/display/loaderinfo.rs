//! `flash.display.LoaderInfo` builtin/prototype

use crate::avm2::activation::Activation;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::method::Method;
use crate::avm2::names::{Namespace, QName};
use crate::avm2::object::{DomainObject, LoaderInfoObject, LoaderStream, Object, TObject};
use crate::avm2::scope::Scope;
use crate::avm2::traits::Trait;
use crate::avm2::value::Value;
use crate::avm2::{AvmString, Error};
use crate::display_object::TDisplayObject;
use gc_arena::{GcCell, MutationContext};

/// Implements `flash.display.LoaderInfo`'s instance constructor.
pub fn instance_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Err("LoaderInfo cannot be constructed".into())
}

/// Implements `flash.display.LoaderInfo`'s class constructor.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

/// `actionScriptVersion` getter
pub fn action_script_version<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(movie, _) => {
                    let library = activation
                        .context
                        .library
                        .library_for_movie_mut(movie.clone());
                    return Ok(library.avm_type().into_avm2_loader_version().into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `applicationDomain` getter
pub fn application_domain<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(movie, _) => {
                    let library = activation
                        .context
                        .library
                        .library_for_movie_mut(movie.clone());
                    return Ok(DomainObject::from_domain(
                        activation.context.gc_context,
                        Some(activation.context.avm2.prototypes().application_domain),
                        library.avm2_domain(),
                    )
                    .into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `bytesTotal` getter
///
/// TODO: This is also the getter for `bytesLoaded` as we don't yet support
/// streaming loads yet. When we do, we'll need another property for this.
pub fn bytes_total<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(movie, _) => {
                    return Ok(movie.compressed_length().into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `content` getter
pub fn content<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(_, root) => {
                    return Ok(root.object2());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `contentType` getter
pub fn content_type<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(_, _) => {
                    return Ok("application/x-shockwave-flash".into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `frameRate` getter
pub fn frame_rate<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(root, _) => {
                    return Ok(root.header().frame_rate.into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `height` getter
pub fn height<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(root, _) => {
                    let y_min = root.header().stage_size.y_min;
                    let y_max = root.header().stage_size.y_max;
                    return Ok((y_max - y_min).to_pixels().into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `isURLInaccessible` getter stub
pub fn is_url_inaccessible<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(false.into())
}

/// `swfVersion` getter
pub fn swf_version<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(root, _) => {
                    return Ok(root.header().version.into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// `url` getter
pub fn url<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(loader_stream) = this.as_loader_stream() {
            match &*loader_stream {
                LoaderStream::SWF(root, _) => {
                    let url = root.url().unwrap_or("").to_string();
                    return Ok(AvmString::new(activation.context.gc_context, url).into());
                }
            }
        }
    }

    Ok(Value::Undefined)
}

/// Derive `LoaderInfoObject` impls.
pub fn loaderinfo_deriver<'gc>(
    base_proto: Object<'gc>,
    activation: &mut Activation<'_, 'gc, '_>,
    class: GcCell<'gc, Class<'gc>>,
    scope: Option<GcCell<'gc, Scope<'gc>>>,
) -> Result<Object<'gc>, Error> {
    LoaderInfoObject::derive(base_proto, activation.context.gc_context, class, scope)
}

/// Construct `LoaderInfo`'s class.
pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.display"), "LoaderInfo"),
        Some(QName::new(Namespace::package("flash.events"), "EventDispatcher").into()),
        Method::from_builtin(instance_init),
        Method::from_builtin(class_init),
        mc,
    );

    let mut write = class.write(mc);

    write.set_attributes(ClassAttributes::SEALED);

    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "actionScriptVersion"),
        Method::from_builtin(action_script_version),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "applicationDomain"),
        Method::from_builtin(application_domain),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "bytesLoaded"),
        Method::from_builtin(bytes_total),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "bytesTotal"),
        Method::from_builtin(bytes_total),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "content"),
        Method::from_builtin(content),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "contentType"),
        Method::from_builtin(content_type),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "frameRate"),
        Method::from_builtin(frame_rate),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "height"),
        Method::from_builtin(height),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "isURLInaccessible"),
        Method::from_builtin(is_url_inaccessible),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "swfVersion"),
        Method::from_builtin(swf_version),
    ));
    write.define_instance_trait(Trait::from_getter(
        QName::new(Namespace::public(), "url"),
        Method::from_builtin(url),
    ));

    class
}