#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unreachable_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(unused_parens)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
use fable_library_rust::NativeArray_::array_from;
use fable_library_rust::String_::fromString;
mod module_2555ccf7 {
    pub mod Documents {
        use super::*;
        use fable_library_rust::Async_::Async;
        use fable_library_rust::DateTime_::DateTime;
        use fable_library_rust::List_::cons;
        use fable_library_rust::List_::empty;
        use fable_library_rust::List_::foldBack;
        use fable_library_rust::List_::item;
        use fable_library_rust::List_::length as length_1;
        use fable_library_rust::List_::ofArray;
        use fable_library_rust::List_::toArray;
        use fable_library_rust::List_::List;
        use fable_library_rust::Map_::find;
        use fable_library_rust::Map_::ofSeq;
        use fable_library_rust::NativeArray_::get_Count;
        use fable_library_rust::NativeArray_::new_array;
        use fable_library_rust::NativeArray_::new_empty;
        use fable_library_rust::NativeArray_::new_init;
        use fable_library_rust::NativeArray_::Array;
        use fable_library_rust::Native_::getZero;
        use fable_library_rust::Native_::unbox;
        use fable_library_rust::Native_::Any;
        use fable_library_rust::Native_::Arc;
        use fable_library_rust::Native_::Func0;
        use fable_library_rust::Native_::Func1;
        use fable_library_rust::Native_::Func2;
        use fable_library_rust::Native_::LrcPtr;
        use fable_library_rust::Native_::MutCell;
        use fable_library_rust::Native_::OnceInit;
        use fable_library_rust::Option_::defaultValue;
        use fable_library_rust::Option_::map;
        use fable_library_rust::Range_::rangeNumeric;
        use fable_library_rust::Seq_::delay;
        use fable_library_rust::Seq_::map as map_1;
        use fable_library_rust::Seq_::ofArray as ofArray_1;
        use fable_library_rust::Seq_::ofList;
        use fable_library_rust::Seq_::toArray as toArray_1;
        use fable_library_rust::String_::append;
        use fable_library_rust::String_::concat;
        use fable_library_rust::String_::contains;
        use fable_library_rust::String_::endsWith3;
        use fable_library_rust::String_::getCharAt;
        use fable_library_rust::String_::getSlice;
        use fable_library_rust::String_::indexOf;
        use fable_library_rust::String_::join;
        use fable_library_rust::String_::lastIndexOf;
        use fable_library_rust::String_::length;
        use fable_library_rust::String_::ofChar;
        use fable_library_rust::String_::printfn;
        use fable_library_rust::String_::replace;
        use fable_library_rust::String_::replicate;
        use fable_library_rust::String_::split;
        use fable_library_rust::String_::sprintf;
        use fable_library_rust::String_::string;
        use fable_library_rust::String_::toLower;
        use fable_library_rust::String_::toString;
        use fable_library_rust::String_::trim;
        use fable_library_rust::String_::trimEndChars;
        use fable_library_rust::String_::trimStartChars;
        use fable_library_rust::TimeSpan_::TimeSpan;
        type ConcurrentStack_1<T> = T;
        use fable_library_rust::System::Collections::Generic::IEnumerable_1;
        use fable_library_rust::System::Exception;
        use fable_library_rust::System::Text::StringBuilder;
        use fable_library_rust::System::Threading::CancellationToken;
        type TaskCanceledException = ();
        pub mod TraceState {
            use super::*;
            pub fn trace_state() -> LrcPtr<
                MutCell<
                    Option<(
                        LrcPtr<Documents::Mut0>,
                        LrcPtr<Documents::Mut1>,
                        LrcPtr<Documents::Mut2>,
                        LrcPtr<Documents::Mut3>,
                        LrcPtr<Documents::Mut4>,
                        Option<i64>,
                    )>,
                >,
            > {
                static trace_state: OnceInit<
                    LrcPtr<
                        MutCell<
                            Option<(
                                LrcPtr<Documents::Mut0>,
                                LrcPtr<Documents::Mut1>,
                                LrcPtr<Documents::Mut2>,
                                LrcPtr<Documents::Mut3>,
                                LrcPtr<Documents::Mut4>,
                                Option<i64>,
                            )>,
                        >,
                    >,
                > = OnceInit::new();
                trace_state
                    .get_or_init(|| {
                        LrcPtr::new(MutCell::new(
                            None::<(
                                LrcPtr<Documents::Mut0>,
                                LrcPtr<Documents::Mut1>,
                                LrcPtr<Documents::Mut2>,
                                LrcPtr<Documents::Mut3>,
                                LrcPtr<Documents::Mut4>,
                                Option<i64>,
                            )>,
                        ))
                    })
                    .clone()
            }
        }
        pub trait IOsEnviron: core::fmt::Debug + core::fmt::Display {
            fn environ(&self) -> LrcPtr<dyn Any>;
        }
        impl<V: IOsEnviron + core::fmt::Debug + core::fmt::Display> IOsEnviron for LrcPtr<V> {
            #[inline]
            fn environ(&self) -> LrcPtr<dyn Any> {
                (**self).environ()
            }
        }
        pub trait IPathJoin: core::fmt::Debug + core::fmt::Display {
            fn join(&self, paths: Array<string>) -> string;
        }
        impl<V: IPathJoin + core::fmt::Debug + core::fmt::Display> IPathJoin for LrcPtr<V> {
            #[inline]
            fn join(&self, paths: Array<string>) -> string {
                (**self).join(paths)
            }
        }
        pub trait IFsExistsSync: core::fmt::Debug + core::fmt::Display {
            fn existsSync(&self, path: string) -> bool;
        }
        impl<V: IFsExistsSync + core::fmt::Debug + core::fmt::Display> IFsExistsSync for LrcPtr<V> {
            #[inline]
            fn existsSync(&self, path: string) -> bool {
                (**self).existsSync(path)
            }
        }
        pub trait IPathDirname: core::fmt::Debug + core::fmt::Display {
            fn dirname(&self, path: string) -> string;
        }
        impl<V: IPathDirname + core::fmt::Debug + core::fmt::Display> IPathDirname for LrcPtr<V> {
            #[inline]
            fn dirname(&self, path: string) -> string {
                (**self).dirname(path)
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US0 {
            US0_0,
            US0_1,
            US0_2,
            US0_3,
            US0_4,
        }
        impl core::fmt::Display for US0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut0 {
            pub l0: MutCell<i64>,
        }
        impl core::fmt::Display for Mut0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub struct Mut1 {
            pub l0: MutCell<Func1<string, ()>>,
        }
        impl core::fmt::Display for Mut1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut2 {
            pub l0: MutCell<bool>,
        }
        impl core::fmt::Display for Mut2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut3 {
            pub l0: MutCell<string>,
        }
        impl core::fmt::Display for Mut3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut4 {
            pub l0: MutCell<Documents::US0>,
        }
        impl core::fmt::Display for Mut4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US1 {
            US1_0(Documents::US0),
            US1_1,
        }
        impl core::fmt::Display for US1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US2 {
            US2_0(i64),
            US2_1,
        }
        impl core::fmt::Display for US2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US3 {
            US3_0,
            US3_1,
            US3_2,
        }
        impl core::fmt::Display for US3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US4 {
            US4_0(Documents::US3),
            US4_1(Documents::US3),
            US4_2(Documents::US3),
            US4_3(Documents::US3),
            US4_4(Documents::US3),
        }
        impl core::fmt::Display for US4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US5 {
            US5_0(string),
            US5_1,
        }
        impl core::fmt::Display for US5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US6 {
            US6_0(std::string::String),
            US6_1,
        }
        impl core::fmt::Display for US6 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US7 {
            US7_0(string),
            US7_1(string),
        }
        impl core::fmt::Display for US7 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US8 {
            US8_0(std::path::PathBuf),
            US8_1(string),
        }
        impl core::fmt::Display for US8 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US9 {
            US9_0(std::path::PathBuf),
            US9_1,
        }
        impl core::fmt::Display for US9 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut5 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<i32>,
            pub l2: MutCell<Array<string>>,
        }
        impl core::fmt::Display for Mut5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut6 {
            pub l0: MutCell<i32>,
        }
        impl core::fmt::Display for Mut6 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US10 {
            US10_0(std::fs::FileType),
            US10_1(std::string::String),
        }
        impl core::fmt::Display for US10 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US11 {
            US11_0,
            US11_1,
            US11_2,
        }
        impl core::fmt::Display for US11 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US12 {
            US12_0(async_walkdir::DirEntry),
            US12_1(std::string::String),
        }
        impl core::fmt::Display for US12 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US13 {
            US13_0(string, Documents::US5),
            US13_1(string),
        }
        impl core::fmt::Display for US13 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US14 {
            US14_0(char, string, LrcPtr<StringBuilder>, i32, i32),
            US14_1(string),
        }
        impl core::fmt::Display for US14 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum UH0 {
            UH0_0,
            UH0_1(char, LrcPtr<Documents::UH0>),
        }
        impl core::fmt::Display for UH0 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum UH1 {
            UH1_0,
            UH1_1(
                Func1<(string, LrcPtr<StringBuilder>, i32, i32), Documents::US14>,
                LrcPtr<Documents::UH1>,
            ),
        }
        impl core::fmt::Display for UH1 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US15 {
            US15_0(string, string, LrcPtr<StringBuilder>, i32, i32),
            US15_1(string),
        }
        impl core::fmt::Display for US15 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US16 {
            US16_0(char),
            US16_1,
        }
        impl core::fmt::Display for US16 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US17 {
            US17_0(string, LrcPtr<StringBuilder>, i32, i32),
            US17_1(string),
        }
        impl core::fmt::Display for US17 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US18 {
            US18_0(
                string,
                Documents::US5,
                string,
                LrcPtr<StringBuilder>,
                i32,
                i32,
            ),
            US18_1(string),
        }
        impl core::fmt::Display for US18 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US19 {
            US19_0(Documents::US16, string, LrcPtr<StringBuilder>, i32, i32),
            US19_1(string),
        }
        impl core::fmt::Display for US19 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US20 {
            US20_0(Documents::US5, string, LrcPtr<StringBuilder>, i32, i32),
            US20_1(string),
        }
        impl core::fmt::Display for US20 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US21 {
            US21_0(Array<string>),
            US21_1(string),
        }
        impl core::fmt::Display for US21 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum UH2 {
            UH2_0,
            UH2_1(string, LrcPtr<Documents::UH2>),
        }
        impl core::fmt::Display for UH2 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US22 {
            US22_0(
                LrcPtr<Documents::UH2>,
                string,
                LrcPtr<StringBuilder>,
                i32,
                i32,
            ),
            US22_1(string),
        }
        impl core::fmt::Display for US22 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum UH3 {
            UH3_0,
            UH3_1(
                Func1<(string, LrcPtr<StringBuilder>, i32, i32), Documents::US15>,
                LrcPtr<Documents::UH3>,
            ),
        }
        impl core::fmt::Display for UH3 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US23 {
            US23_0(std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>),
            US23_1(std::string::String),
        }
        impl core::fmt::Display for US23 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US24 {
            US24_0(
                std::sync::Arc<
                    std::sync::Mutex<
                        std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                    >,
                >,
            ),
            US24_1,
        }
        impl core::fmt::Display for US24 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US25 {
            US25_0(std::string::String),
            US25_1(std::string::String),
        }
        impl core::fmt::Display for US25 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US26 {
            US26_0(Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>),
            US26_1,
        }
        impl core::fmt::Display for US26 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US27 {
            US27_0(std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>),
            US27_1,
        }
        impl core::fmt::Display for US27 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US28 {
            US28_0(std::process::Output),
            US28_1(std::string::String),
        }
        impl core::fmt::Display for US28 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US29 {
            US29_0(i32),
            US29_1,
        }
        impl core::fmt::Display for US29 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US30 {
            US30_0(Func1<(i32, string, bool), Arc<Async<()>>>),
            US30_1,
        }
        impl core::fmt::Display for US30 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US31 {
            US31_0(CancellationToken),
            US31_1,
        }
        impl core::fmt::Display for US31 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US32 {
            US32_0(Result<string, LrcPtr<(string, string)>>),
            US32_1,
        }
        impl core::fmt::Display for US32 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum UH5 {
            UH5_0,
            UH5_1(
                string,
                string,
                Func2<string, string, Documents::US32>,
                LrcPtr<Documents::UH5>,
            ),
        }
        impl core::fmt::Display for UH5 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum UH4 {
            UH4_0,
            UH4_1(LrcPtr<Documents::UH5>, LrcPtr<Documents::UH4>),
        }
        impl core::fmt::Display for UH4 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US33 {
            US33_0(string),
            US33_1(std::string::String),
        }
        impl core::fmt::Display for US33 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US34 {
            US34_0(u64),
            US34_1(std::string::String),
        }
        impl core::fmt::Display for US34 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub enum US35 {
            US35_0(i32, string),
            US35_1(i32, string),
        }
        impl core::fmt::Display for US35 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut7 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<i32>,
        }
        impl core::fmt::Display for Mut7 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug, Hash, PartialEq, PartialOrd)]
        pub struct Mut8 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<string>,
            pub l2: MutCell<i32>,
            pub l3: MutCell<i32>,
        }
        impl core::fmt::Display for Mut8 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub struct Mut9 {
            pub l0: MutCell<i32>,
            pub l1: MutCell<Vec<Option<Result<string, LrcPtr<(string, string)>>>>>,
        }
        impl core::fmt::Display for Mut9 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        #[derive(Clone, Debug)]
        pub enum US36 {
            US36_0(
                Vec<
                    Result<
                        LrcPtr<(
                            string,
                            Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                        )>,
                        std::string::String,
                    >,
                >,
            ),
            US36_1(std::string::String),
        }
        impl core::fmt::Display for US36 {
            fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                write!(f, "{}", core::any::type_name::<Self>())
            }
        }
        pub fn method0() -> clap::Command {
            let v1_1: string = string("r#\"command\"#");
            let v2: &'static str = r#"command"#;
            let v4: clap::Command = clap::Command::new(v2);
            let v6: string = string("r#\"source-dir\"#");
            let v7: &'static str = r#"source-dir"#;
            let v9: clap::Arg = clap::Arg::new(v7);
            let v11: clap::Arg = v9.short('s');
            let v12: string = string("r#\"source-dir\"#");
            let v13: &'static str = r#"source-dir"#;
            let v15: clap::Arg = v11.long(v13);
            let v17: clap::Arg = v15.required(true);
            let v19: clap::Command = clap::Command::arg(v4, v17);
            let v21: string = string("r#\"dist-dir\"#");
            let v22: &'static str = r#"dist-dir"#;
            let v24: clap::Arg = clap::Arg::new(v22);
            let v26: clap::Arg = v24.short('d');
            let v27: string = string("r#\"dist-dir\"#");
            let v28: &'static str = r#"dist-dir"#;
            let v30: clap::Arg = v26.long(v28);
            let v32: clap::Arg = v30.required(true);
            let v34: clap::Command = clap::Command::arg(v19, v32);
            let v36: string = string("r#\"cache-dir\"#");
            let v37: &'static str = r#"cache-dir"#;
            let v39: clap::Arg = clap::Arg::new(v37);
            let v41: clap::Arg = v39.short('c');
            let v42: string = string("r#\"cache-dir\"#");
            let v43: &'static str = r#"cache-dir"#;
            let v45: clap::Arg = v41.long(v43);
            let v47: clap::Arg = v45.required(true);
            let v49: clap::Command = clap::Command::arg(v34, v47);
            let v51: string = string("r#\"hangul-spec\"#");
            let v52: &'static str = r#"hangul-spec"#;
            let v54: clap::Arg = clap::Arg::new(v52);
            let v56: clap::Arg = v54.short('h');
            let v57: string = string("r#\"hangul-spec\"#");
            let v58: &'static str = r#"hangul-spec"#;
            let v60: clap::Arg = v56.long(v58);
            let v62: clap::Arg = v60.required(true);
            clap::Command::arg(v49, v62)
        }
        pub fn closure0(unitVar: (), unitVar_1: ()) {
            let v1_1: bool = true;
            () //;
        } /* /*;
          {
              let v4: string = string("*/ #[test] fn verify_app() { //");
              let v5: bool = */
        #[test]
        fn verify_app() {
            //;
            let v6: clap::Command = Documents::method0();
            clap::Command::debug_assert(v6);
            {
                //;
                ()
            }
        }
        pub fn method4(v0_1: string) -> string {
            v0_1
        }
        pub fn method5() -> string {
            string("")
        }
        pub fn closure3(unitVar: (), v0_1: string) -> Documents::US5 {
            Documents::US5::US5_0(v0_1)
        }
        pub fn method6() -> Func1<string, Documents::US5> {
            Func1::new(move |v: string| Documents::closure3((), v))
        }
        pub fn method3(v0_1: string) -> string {
            let v2: string = Documents::method4(v0_1);
            let v4: Result<std::string::String, std::env::VarError> = std::env::var(&*v2);
            let v6: bool = true;
            let _result_map_ = v4.map(|x| {
                //;
                let v8: std::string::String = x;
                let v10: string = fable_library_rust::String_::fromString(v8);
                let v12: bool = true;
                v10
            });
            let v14: Result<string, std::env::VarError> = _result_map_;
            let v15: string = Documents::method5();
            v14.unwrap_or(v15)
        }
        pub fn method2() -> (Documents::US1, Documents::US2) {
            let v1_1: string = Documents::method3(string("TRACE_LEVEL"));
            let v6: Documents::US1 = if string("Verbose") == (v1_1.clone()) {
                Documents::US1::US1_0(Documents::US0::US0_0)
            } else {
                Documents::US1::US1_1
            };
            (
                match &v6 {
                    Documents::US1::US1_0(v6_0_0) => Documents::US1::US1_0(
                        match &v6 {
                            Documents::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                    ),
                    _ => {
                        let v13: Documents::US1 = if string("Debug") == (v1_1.clone()) {
                            Documents::US1::US1_0(Documents::US0::US0_1)
                        } else {
                            Documents::US1::US1_1
                        };
                        match &v13 {
                            Documents::US1::US1_0(v13_0_0) => Documents::US1::US1_0(
                                match &v13 {
                                    Documents::US1::US1_0(x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone(),
                            ),
                            _ => {
                                let v20: Documents::US1 = if string("Info") == (v1_1.clone()) {
                                    Documents::US1::US1_0(Documents::US0::US0_2)
                                } else {
                                    Documents::US1::US1_1
                                };
                                match &v20 {
                                    Documents::US1::US1_0(v20_0_0) => Documents::US1::US1_0(
                                        match &v20 {
                                            Documents::US1::US1_0(x) => x.clone(),
                                            _ => unreachable!(),
                                        }
                                        .clone(),
                                    ),
                                    _ => {
                                        let v27: Documents::US1 =
                                            if string("Warning") == (v1_1.clone()) {
                                                Documents::US1::US1_0(Documents::US0::US0_3)
                                            } else {
                                                Documents::US1::US1_1
                                            };
                                        match &v27 {
                                            Documents::US1::US1_0(v27_0_0) => {
                                                Documents::US1::US1_0(
                                                    match &v27 {
                                                        Documents::US1::US1_0(x) => x.clone(),
                                                        _ => unreachable!(),
                                                    }
                                                    .clone(),
                                                )
                                            }
                                            _ => {
                                                let v34: Documents::US1 =
                                                    if string("Critical") == (v1_1.clone()) {
                                                        Documents::US1::US1_0(Documents::US0::US0_4)
                                                    } else {
                                                        Documents::US1::US1_1
                                                    };
                                                match &v34 {
                                                    Documents::US1::US1_0(v34_0_0) => {
                                                        Documents::US1::US1_0(
                                                            match &v34 {
                                                                Documents::US1::US1_0(x) => {
                                                                    x.clone()
                                                                }
                                                                _ => unreachable!(),
                                                            }
                                                            .clone(),
                                                        )
                                                    }
                                                    _ => Documents::US1::US1_1,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                },
                if (Documents::method3(string("AUTOMATION"))) != string("True") {
                    Documents::US2::US2_1
                } else {
                    Documents::US2::US2_0({
                        let _arg: DateTime = DateTime::now();
                        _arg.ticks()
                    })
                },
            )
        }
        pub fn closure4(unitVar: (), v0_1: string) {
            ();
        }
        pub fn method1(
            v0_1: Documents::US0,
        ) -> (
            LrcPtr<Documents::Mut0>,
            LrcPtr<Documents::Mut1>,
            LrcPtr<Documents::Mut2>,
            LrcPtr<Documents::Mut3>,
            LrcPtr<Documents::Mut4>,
            Option<i64>,
        ) {
            let patternInput: (Documents::US1, Documents::US2) = Documents::method2();
            let _v1: (Documents::US1, Documents::US2) =
                (patternInput.0.clone(), patternInput.1.clone());
            let v132: Documents::US2 = _v1.1.clone();
            let v131: Documents::US1 = _v1.0.clone();
            (
                LrcPtr::new(Documents::Mut0 {
                    l0: MutCell::new(1_i64),
                }),
                LrcPtr::new(Documents::Mut1 {
                    l0: MutCell::new(Func1::new(move |v: string| Documents::closure4((), v))),
                }),
                LrcPtr::new(Documents::Mut2 {
                    l0: MutCell::new(true),
                }),
                LrcPtr::new(Documents::Mut3 {
                    l0: MutCell::new(string("")),
                }),
                LrcPtr::new(Documents::Mut4 {
                    l0: MutCell::new(match &v131 {
                        Documents::US1::US1_0(v131_0_0) => match &v131 {
                            Documents::US1::US1_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        _ => v0_1.clone(),
                    }),
                }),
                match &v132 {
                    Documents::US2::US2_0(v132_0_0) => Some(match &v132 {
                        Documents::US2::US2_0(x) => x.clone(),
                        _ => unreachable!(),
                    }),
                    _ => None::<i64>,
                },
            )
        }
        pub fn closure2(unitVar: (), unitVar_1: ()) {
            if Documents::TraceState::trace_state().get().clone().is_none() {
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::method1(Documents::US0::US0_2);
                Documents::TraceState::trace_state().set(Some((
                    patternInput.0.clone(),
                    patternInput.1.clone(),
                    patternInput.2.clone(),
                    patternInput.3.clone(),
                    patternInput.4.clone(),
                    patternInput.5.clone(),
                )));
                ()
            };
        }
        pub fn closure6(unitVar: (), unitVar_1: ()) {
            if Documents::TraceState::trace_state().get().clone().is_none() {
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::method1(Documents::US0::US0_0);
                Documents::TraceState::trace_state().set(Some((
                    patternInput.0.clone(),
                    patternInput.1.clone(),
                    patternInput.2.clone(),
                    patternInput.3.clone(),
                    patternInput.4.clone(),
                    patternInput.5.clone(),
                )));
                ()
            };
        }
        pub fn method7(v0_1: Documents::US0) -> bool {
            let v3: () = {
                Documents::closure6((), ());
                ()
            };
            let patternInput: (
                LrcPtr<Documents::Mut0>,
                LrcPtr<Documents::Mut1>,
                LrcPtr<Documents::Mut2>,
                LrcPtr<Documents::Mut3>,
                LrcPtr<Documents::Mut4>,
                Option<i64>,
            ) = Documents::TraceState::trace_state().get().clone().unwrap();
            let v35: Documents::US0 = (patternInput.4.clone()).l0.get().clone();
            if ((patternInput.2.clone()).l0.get().clone()) == false {
                false
            } else {
                (find(
                    v0_1,
                    ofSeq(ofList(ofArray(new_array(&[
                        LrcPtr::new((Documents::US0::US0_0, 0_i32)),
                        LrcPtr::new((Documents::US0::US0_1, 1_i32)),
                        LrcPtr::new((Documents::US0::US0_2, 2_i32)),
                        LrcPtr::new((Documents::US0::US0_3, 3_i32)),
                        LrcPtr::new((Documents::US0::US0_4, 4_i32)),
                    ])))),
                )) >= (find(
                    v35,
                    ofSeq(ofList(ofArray(new_array(&[
                        LrcPtr::new((Documents::US0::US0_0, 0_i32)),
                        LrcPtr::new((Documents::US0::US0_1, 1_i32)),
                        LrcPtr::new((Documents::US0::US0_2, 2_i32)),
                        LrcPtr::new((Documents::US0::US0_3, 3_i32)),
                        LrcPtr::new((Documents::US0::US0_4, 4_i32)),
                    ])))),
                ))
            }
        }
        pub fn closure7(unitVar: (), v0_1: i64) -> Documents::US2 {
            Documents::US2::US2_0(v0_1)
        }
        pub fn method9() -> Func1<i64, Documents::US2> {
            Func1::new(move |v: i64| Documents::closure7((), v))
        }
        pub fn method10() -> string {
            string("hh:mm:ss")
        }
        pub fn method11() -> string {
            string("HH:mm:ss")
        }
        pub fn method8(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
        ) -> string {
            let v20: Documents::US2 =
                defaultValue(Documents::US2::US2_1, map(Documents::method9(), v5));
            let v117: DateTime = match &v20 {
                Documents::US2::US2_0(v20_0_0) => {
                    let v77: TimeSpan = TimeSpan::new_ticks(
                        ({
                            let _arg: DateTime = DateTime::now();
                            _arg.ticks()
                        }) - (match &v20 {
                            Documents::US2::US2_0(x) => x.clone(),
                            _ => unreachable!(),
                        }),
                    );
                    DateTime::new_ymdhms_milli(
                        1_i32,
                        1_i32,
                        1_i32,
                        v77.hours(),
                        v77.minutes(),
                        v77.seconds(),
                        v77.milliseconds(),
                    )
                }
                _ => DateTime::now(),
            };
            let v118: string = Documents::method10();
            let provider: string = if (v118.clone()) == string("") {
                string("M-d-y hh:mm:ss tt")
            } else {
                v118
            };
            v117.toString(provider)
        }
        pub fn method14() -> string {
            string("")
        }
        pub fn closure8(v0_1: LrcPtr<Documents::Mut3>, v1_1: string, unitVar: ()) {
            let v3: string = append((v0_1.l0.get().clone()), (v1_1));
            v0_1.l0.set(v3);
            ()
        }
        pub fn method13(v0_1: char) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v8: () = {
                Documents::closure8(v2.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method15() -> string {
            string("\u{001b}[0m")
        }
        pub fn method12() -> string {
            let v6: string = Documents::method13(getCharAt(toLower(string("Info")), 0_i32));
            let v9: &str = inline_colorization::color_bright_green;
            let v12: &str = &*v6;
            let v35: &str = inline_colorization::color_reset;
            let v37: std::string::String = format!("{}{}{}", v9, v12, v35);
            fable_library_rust::String_::fromString(v37)
        }
        pub fn method17(v0_1: Array<string>) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("args"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v38: () = {
                Documents::closure8(v2.clone(), sprintf!("{:?}", v0_1), ());
                ()
            };
            let v47: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method18(v0_1: string) -> string {
            trimEndChars(
                trimStartChars(v0_1, toArray(empty::<char>())),
                toArray(ofArray(new_array(&[' ', '/']))),
            )
        }
        pub fn method16(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: Array<string>,
        ) -> string {
            let v9: string = Documents::method17(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.main"),
                v9
            ))
        }
        pub fn closure9(v0_1: LrcPtr<Documents::Mut0>, unitVar: ()) {
            let v2: i64 = (v0_1.l0.get().clone()) + 1_i64;
            v0_1.l0.set(v2);
            ()
        }
        pub fn closure11(v0_1: string, unitVar: ()) {
            printfn!("{0}", v0_1);
        }
        pub fn closure10(unitVar: (), v0_1: string) {
            let v3: () = {
                Documents::closure11(v0_1, ());
                ()
            };
            ()
        }
        pub fn method19(v0_1: string) {
            let v3: () = {
                Documents::closure6((), ());
                ()
            };
            let patternInput: (
                LrcPtr<Documents::Mut0>,
                LrcPtr<Documents::Mut1>,
                LrcPtr<Documents::Mut2>,
                LrcPtr<Documents::Mut3>,
                LrcPtr<Documents::Mut4>,
                Option<i64>,
            ) = Documents::TraceState::trace_state().get().clone().unwrap();
            let v37: () = {
                Documents::closure9(patternInput.0.clone(), ());
                ()
            };
            println!("{}", v0_1.clone());
            ((patternInput.1.clone()).l0.get().clone())(v0_1)
        }
        pub fn closure5(v0_1: Array<string>, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_2) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method16(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method12(),
                    v0_1,
                ))
            };
        }
        pub fn method20() -> string {
            string("source-dir")
        }
        pub fn closure12(unitVar: (), v0_1: std::string::String) -> Documents::US6 {
            Documents::US6::US6_0(v0_1)
        }
        pub fn method21() -> Func1<std::string::String, Documents::US6> {
            Func1::new(move |v: std::string::String| Documents::closure12((), v))
        }
        pub fn method22() -> string {
            string("dist-dir")
        }
        pub fn method23() -> string {
            string("cache-dir")
        }
        pub fn method24() -> string {
            string("hangul-spec")
        }
        pub fn method26(v0_1: string, v1_1: string) -> string {
            let v5: &str = &*v0_1;
            let v29: std::string::String = String::from(v5);
            let v53: std::path::PathBuf = std::path::PathBuf::from(v29);
            let v77: &str = &*v1_1;
            let v101: std::string::String = String::from(v77);
            let v124: std::path::PathBuf = v53.join(v101);
            let v127: std::path::Display = v124.display();
            let v151: std::string::String = format!("{}", v127);
            fable_library_rust::String_::fromString(v151)
        }
        pub fn method28(v0_1: string) -> bool {
            let v4: &str = &*v0_1;
            let v28: std::string::String = String::from(v4);
            let v69: std::path::PathBuf = std::path::PathBuf::from(v28);
            if v69.clone().exists() {
                v69.is_dir()
            } else {
                false
            }
        }
        pub fn method29(v0_1: string) -> Option<string> {
            let v4: &str = &*v0_1;
            let v28: std::string::String = String::from(v4);
            let v52: std::path::PathBuf = std::path::PathBuf::from(v28);
            let v75: Option<std::path::PathBuf> = v52.parent().map(std::path::PathBuf::from);
            let v77: bool = true;
            let _optionm_map_ = v75.map(|x| {
                //;
                let v79: std::path::PathBuf = x;
                let v82: std::path::Display = v79.display();
                let v106: std::string::String = format!("{}", v82);
                let v129: string = fable_library_rust::String_::fromString(v106);
                let v131: bool = true;
                v129
            });
            _optionm_map_
        }
        pub fn method30(v0_1: string, v1_1: string, v2: string) -> Documents::US7 {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<string> = MutCell::new(v2.clone());
            '_method30: loop {
                break '_method30 (if Documents::method28(Documents::method26(
                    v2.get().clone(),
                    v0_1.get().clone(),
                )) {
                    Documents::US7::US7_0(v2.get().clone())
                } else {
                    let v6: Option<string> = Documents::method29(v2.get().clone());
                    let v20: Documents::US5 =
                        defaultValue(Documents::US5::US5_1, map(Documents::method6(), v6));
                    match &v20 {
                        Documents::US5::US5_0(v20_0_0) => {
                            let v0_1_temp: string = v0_1.get().clone();
                            let v1_1_temp: string = v1_1.get().clone();
                            let v2_temp: string = match &v20 {
                                Documents::US5::US5_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            continue '_method30;
                        }
                        _ => Documents::US7::US7_1(sprintf!(
                            "No parent for {} \'{}\' at \'{}\' (until \'{}\')",
                            string("dir"),
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone()
                        )),
                    }
                });
            }
        }
        pub fn method27(v0_1: string, v1_1: string) -> Documents::US7 {
            if Documents::method28(Documents::method26(v1_1.clone(), v0_1.clone())) {
                Documents::US7::US7_0(v1_1.clone())
            } else {
                let v5: Option<string> = Documents::method29(v1_1.clone());
                let v19: Documents::US5 =
                    defaultValue(Documents::US5::US5_1, map(Documents::method6(), v5));
                match &v19 {
                    Documents::US5::US5_0(v19_0_0) => Documents::method30(
                        v0_1.clone(),
                        v1_1.clone(),
                        match &v19 {
                            Documents::US5::US5_0(x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                    ),
                    _ => Documents::US7::US7_1(sprintf!(
                        "No parent for {} \'{}\' at \'{}\' (until \'{}\')",
                        string("dir"),
                        v0_1.clone(),
                        v1_1.clone(),
                        v1_1.clone()
                    )),
                }
            }
        }
        pub fn method31() -> string {
            let v6: string = Documents::method13(getCharAt(toLower(string("Warning")), 0_i32));
            let v9: &str = inline_colorization::color_yellow;
            let v12: &str = &*v6;
            let v35: &str = inline_colorization::color_reset;
            let v37: std::string::String = format!("{}{}{}", v9, v12, v35);
            fable_library_rust::String_::fromString(v37)
        }
        pub fn method33(v0_1: string) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("error"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v35: () = {
                Documents::closure8(v2.clone(), v0_1, ());
                ()
            };
            let v44: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method32(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
        ) -> string {
            let v9: string = Documents::method33(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("file_system.get_workspace_root"),
                v9
            ))
        }
        pub fn closure13(v0_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_3) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method32(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method31(),
                    v0_1,
                ))
            };
        }
        pub fn method34() -> string {
            let v2: Result<std::path::PathBuf, std::io::Error> = std::env::current_dir();
            let v5: std::path::PathBuf = v2.unwrap();
            let v19: std::path::Display = v5.display();
            let v43: std::string::String = format!("{}", v19);
            fable_library_rust::String_::fromString(v43)
        }
        pub fn method40(v0_1: std::io::Error) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v5: std::string::String = format!("{:#?}", v0_1);
            let v38: () = {
                Documents::closure8(v2.clone(), fable_library_rust::String_::fromString(v5), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn closure14(unitVar: (), v0_1: std::io::Error) -> string {
            Documents::method40(v0_1)
        }
        pub fn method39() -> Func1<std::io::Error, string> {
            Func1::new(move |v: std::io::Error| Documents::closure14((), v))
        }
        pub fn closure15(unitVar: (), v0_1: std::path::PathBuf) -> Documents::US8 {
            Documents::US8::US8_0(v0_1)
        }
        pub fn method41() -> Func1<std::path::PathBuf, Documents::US8> {
            Func1::new(move |v: std::path::PathBuf| Documents::closure15((), v))
        }
        pub fn closure16(unitVar: (), v0_1: string) -> Documents::US8 {
            Documents::US8::US8_1(v0_1)
        }
        pub fn method42() -> Func1<string, Documents::US8> {
            Func1::new(move |v: string| Documents::closure16((), v))
        }
        pub fn method45(v0_1: string) -> string {
            let v4: &str = &*v0_1;
            let v28: std::string::String = String::from(v4);
            let v52: std::path::PathBuf = std::path::PathBuf::from(v28);
            let v75: Option<&std::ffi::OsStr> = v52.file_name();
            let v77: bool = true;
            let _optionm_map_ = v75.map(|x| {
                //;
                let v79: &std::ffi::OsStr = x;
                let v81: std::ffi::OsString = v79.to_os_string();
                let v83: Option<&str> = v81.to_str();
                let v85: &str = v83.unwrap();
                let v88: std::string::String = String::from(v85);
                let v111: string = fable_library_rust::String_::fromString(v88);
                let v113: bool = true;
                v111
            });
            let v115: Option<string> = _optionm_map_;
            let v129: Documents::US5 =
                defaultValue(Documents::US5::US5_1, map(Documents::method6(), v115));
            match &v129 {
                Documents::US5::US5_0(v129_0_0) => match &v129 {
                    Documents::US5::US5_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => string(""),
            }
        }
        pub fn method46(v0_1: string) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v8: () = {
                Documents::closure8(v2.clone(), v0_1, ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method44(
            v0_1: string,
            v1_1: Func2<u8, string, Result<std::path::PathBuf, std::io::Error>>,
            v2: u8,
            v3: string,
            v4: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v5: string = Documents::method45(v4.clone());
            let v6: Option<string> = Documents::method29(v4.clone());
            let v20: Documents::US5 =
                defaultValue(Documents::US5::US5_1, map(Documents::method6(), v6));
            let v24: string = Documents::method46(v3);
            if (v2) >= 11_u8 {
                let v26: string = sprintf!(
                    "file_system.read_link / path: {} / n: {} / path\': {} / name: {}",
                    v0_1.clone(),
                    v2,
                    v4.clone(),
                    v5.clone()
                );
                let v29: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v26);
                Err(v29)
            } else {
                if let Documents::US5::US5_0(v20_0_0) = &v20 {
                    if (v4.clone()) != string("") {
                        let v72: Result<std::path::PathBuf, std::io::Error> =
                            v1_1((v2) + 1_u8, v20_0_0.clone());
                        let v73 = Documents::method39();
                        let v85: Result<std::path::PathBuf, string> = v72.map_err(|x| v73(x));
                        let v88 = Documents::method41();
                        let v89 = Documents::method42();
                        let v90: Documents::US8 = match &v85 {
                            Err(v85_1_0) => v89(v85_1_0.clone()),
                            Ok(v85_0_0) => v88(v85_0_0.clone()),
                        };
                        match &v90 {
                            Documents::US8::US8_0(v90_0_0) => {
                                let v119: string = Documents::method26(
                                    toString(v90_0_0.clone().display()),
                                    v5.clone(),
                                );
                                let v122: &str = &*v119;
                                let v146: std::string::String = String::from(v122);
                                let v170: std::path::PathBuf = std::path::PathBuf::from(v146);
                                Ok(v170)
                            }
                            Documents::US8::US8_1(v90_1_0) => {
                                let v207: string = sprintf!(
                                    "file_system.read_link / error\': {} / error: {} / name: {}",
                                    v90_1_0.clone(),
                                    v24.clone(),
                                    v5.clone()
                                );
                                let v210: std::io::Error =
                                    std::io::Error::new(std::io::ErrorKind::Other, &*v207);
                                Err(v210)
                            }
                        }
                    } else {
                        let v248: string =
                            sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                     v0_1.clone(), v24.clone(), v4.clone(),
                                     v5.clone());
                        let v251: std::io::Error =
                            std::io::Error::new(std::io::ErrorKind::Other, &*v248);
                        Err(v251)
                    }
                } else {
                    let v288: string =
                        sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                 v0_1, v24.clone(), v4, v5.clone());
                    let v291: std::io::Error =
                        std::io::Error::new(std::io::ErrorKind::Other, &*v288);
                    Err(v291)
                }
            }
        }
        pub fn method43(
            v0_1: string,
            v1_1: u8,
            v2: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v5: Result<std::path::PathBuf, std::io::Error> = std::fs::read_link(&*v2.clone());
            let v6 = Documents::method39();
            let v18: Result<std::path::PathBuf, string> = v5.map_err(|x| v6(x));
            let v21 = Documents::method41();
            let v22 = Documents::method42();
            let v23: Documents::US8 = match &v18 {
                Err(v18_1_0) => v22(v18_1_0.clone()),
                Ok(v18_0_0) => v21(v18_0_0.clone()),
            };
            match &v23 {
                Documents::US8::US8_0(v23_0_0) => Ok(v23_0_0.clone()),
                Documents::US8::US8_1(v23_1_0) => Documents::method44(
                    v0_1.clone(),
                    Func2::new({
                        let v0_1 = v0_1.clone();
                        move |b0: u8, b1: string| {
                            (Func1::new({
                                let v0_1 = v0_1.clone();
                                move |v: u8| Documents::closure17(v0_1.clone(), v)
                            }))(b0)(b1)
                        }
                    }),
                    v1_1,
                    v23_1_0.clone(),
                    v2.clone(),
                ),
            }
        }
        pub fn closure18(
            v0_1: string,
            v1_1: u8,
            v2: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Documents::method43(v0_1, v1_1, v2)
        }
        pub fn closure17(
            v0_1: string,
            v1_1: u8,
        ) -> Func1<string, Result<std::path::PathBuf, std::io::Error>> {
            Func1::new({
                let v0_1 = v0_1.clone();
                let v1_1 = v1_1.clone();
                move |v: string| Documents::closure18(v0_1.clone(), v1_1, v)
            })
        }
        pub fn method47(
            v0_1: string,
            v1_1: Func2<u8, string, Result<std::path::PathBuf, std::io::Error>>,
            v2: u8,
            v3: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v4: string = Documents::method45(v0_1.clone());
            let v5: Option<string> = Documents::method29(v0_1.clone());
            let v19: Documents::US5 =
                defaultValue(Documents::US5::US5_1, map(Documents::method6(), v5));
            let v23: string = Documents::method46(v3);
            if (v2) >= 11_u8 {
                let v25: string = sprintf!(
                    "file_system.read_link / path: {} / n: {} / path\': {} / name: {}",
                    v0_1.clone(),
                    v2,
                    v0_1.clone(),
                    v4.clone()
                );
                let v28: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v25);
                Err(v28)
            } else {
                if let Documents::US5::US5_0(v19_0_0) = &v19 {
                    if (v0_1.clone()) != string("") {
                        let v71: Result<std::path::PathBuf, std::io::Error> =
                            v1_1((v2) + 1_u8, v19_0_0.clone());
                        let v72 = Documents::method39();
                        let v84: Result<std::path::PathBuf, string> = v71.map_err(|x| v72(x));
                        let v87 = Documents::method41();
                        let v88 = Documents::method42();
                        let v89: Documents::US8 = match &v84 {
                            Err(v84_1_0) => v88(v84_1_0.clone()),
                            Ok(v84_0_0) => v87(v84_0_0.clone()),
                        };
                        match &v89 {
                            Documents::US8::US8_0(v89_0_0) => {
                                let v118: string = Documents::method26(
                                    toString(v89_0_0.clone().display()),
                                    v4.clone(),
                                );
                                let v121: &str = &*v118;
                                let v145: std::string::String = String::from(v121);
                                let v169: std::path::PathBuf = std::path::PathBuf::from(v145);
                                Ok(v169)
                            }
                            Documents::US8::US8_1(v89_1_0) => {
                                let v206: string = sprintf!(
                                    "file_system.read_link / error\': {} / error: {} / name: {}",
                                    v89_1_0.clone(),
                                    v23.clone(),
                                    v4.clone()
                                );
                                let v209: std::io::Error =
                                    std::io::Error::new(std::io::ErrorKind::Other, &*v206);
                                Err(v209)
                            }
                        }
                    } else {
                        let v247: string =
                            sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                     v0_1.clone(), v23.clone(), v0_1.clone(),
                                     v4.clone());
                        let v250: std::io::Error =
                            std::io::Error::new(std::io::ErrorKind::Other, &*v247);
                        Err(v250)
                    }
                } else {
                    let v287: string =
                        sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                 v0_1.clone(), v23.clone(), v0_1, v4.clone());
                    let v290: std::io::Error =
                        std::io::Error::new(std::io::ErrorKind::Other, &*v287);
                    Err(v290)
                }
            }
        }
        pub fn method38(v0_1: string, v1_1: u8) -> Result<std::path::PathBuf, std::io::Error> {
            let v4: Result<std::path::PathBuf, std::io::Error> = std::fs::read_link(&*v0_1.clone());
            let v5 = Documents::method39();
            let v17: Result<std::path::PathBuf, string> = v4.map_err(|x| v5(x));
            let v20 = Documents::method41();
            let v21 = Documents::method42();
            let v22: Documents::US8 = match &v17 {
                Err(v17_1_0) => v21(v17_1_0.clone()),
                Ok(v17_0_0) => v20(v17_0_0.clone()),
            };
            match &v22 {
                Documents::US8::US8_0(v22_0_0) => Ok(v22_0_0.clone()),
                Documents::US8::US8_1(v22_1_0) => Documents::method47(
                    v0_1.clone(),
                    Func2::new({
                        let v0_1 = v0_1.clone();
                        move |b0: u8, b1: string| {
                            (Func1::new({
                                let v0_1 = v0_1.clone();
                                move |v: u8| Documents::closure17(v0_1.clone(), v)
                            }))(b0)(b1)
                        }
                    }),
                    v1_1,
                    v22_1_0.clone(),
                ),
            }
        }
        pub fn method49(v0_1: ()) -> i32 {
            unbox::<i32>(&getZero())
        }
        pub fn method50() -> i32 {
            unbox::<i32>(&getZero())
        }
        pub fn method51(v0_1: i32, v1_1: i32) -> bool {
            unbox::<bool>(&getZero())
        }
        pub fn method53(
            v0_1: string,
            v1_1: Func2<u8, string, Result<std::path::PathBuf, std::io::Error>>,
            v2: u8,
            v3: std::io::Error,
            v4: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v5: string = Documents::method45(v4.clone());
            let v6: Option<string> = Documents::method29(v4.clone());
            let v20: Documents::US5 =
                defaultValue(Documents::US5::US5_1, map(Documents::method6(), v6));
            let v24: string = Documents::method40(v3);
            if (v2) >= 11_u8 {
                let v26: string = sprintf!(
                    "file_system.read_link / path: {} / n: {} / path\': {} / name: {}",
                    v0_1.clone(),
                    v2,
                    v4.clone(),
                    v5.clone()
                );
                let v29: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v26);
                Err(v29)
            } else {
                if let Documents::US5::US5_0(v20_0_0) = &v20 {
                    if (v4.clone()) != string("") {
                        let v72: Result<std::path::PathBuf, std::io::Error> =
                            v1_1((v2) + 1_u8, v20_0_0.clone());
                        let v73 = Documents::method39();
                        let v85: Result<std::path::PathBuf, string> = v72.map_err(|x| v73(x));
                        let v88 = Documents::method41();
                        let v89 = Documents::method42();
                        let v90: Documents::US8 = match &v85 {
                            Err(v85_1_0) => v89(v85_1_0.clone()),
                            Ok(v85_0_0) => v88(v85_0_0.clone()),
                        };
                        match &v90 {
                            Documents::US8::US8_0(v90_0_0) => {
                                let v119: string = Documents::method26(
                                    toString(v90_0_0.clone().display()),
                                    v5.clone(),
                                );
                                let v122: &str = &*v119;
                                let v146: std::string::String = String::from(v122);
                                let v170: std::path::PathBuf = std::path::PathBuf::from(v146);
                                Ok(v170)
                            }
                            Documents::US8::US8_1(v90_1_0) => {
                                let v207: string = sprintf!(
                                    "file_system.read_link / error\': {} / error: {} / name: {}",
                                    v90_1_0.clone(),
                                    v24.clone(),
                                    v5.clone()
                                );
                                let v210: std::io::Error =
                                    std::io::Error::new(std::io::ErrorKind::Other, &*v207);
                                Err(v210)
                            }
                        }
                    } else {
                        let v248: string =
                            sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                     v0_1.clone(), v24.clone(), v4.clone(),
                                     v5.clone());
                        let v251: std::io::Error =
                            std::io::Error::new(std::io::ErrorKind::Other, &*v248);
                        Err(v251)
                    }
                } else {
                    let v288: string =
                        sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                 v0_1, v24.clone(), v4, v5.clone());
                    let v291: std::io::Error =
                        std::io::Error::new(std::io::ErrorKind::Other, &*v288);
                    Err(v291)
                }
            }
        }
        pub fn method52(
            v0_1: string,
            v1_1: u8,
            v2: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v30: i32 = Documents::method49(getZero());
            let v32: bool = Documents::method51(Documents::method50(), v30);
            if v32 {
                let v34: () = getZero();
                let v86: std::path::PathBuf = getZero();
                Ok(v86)
            } else {
                let v103: string =
                    sprintf!("file_system.read_link / Fsharp / The file or directory is not a reparse point. / path: {} / result: {} / path\': {} / n: {}",
                             v0_1.clone(), v32, v2.clone(), v1_1);
                Documents::method53(
                    v0_1.clone(),
                    Func2::new({
                        let v0_1 = v0_1.clone();
                        move |b0: u8, b1: string| {
                            (Func1::new({
                                let v0_1 = v0_1.clone();
                                move |v: u8| Documents::closure19(v0_1.clone(), v)
                            }))(b0)(b1)
                        }
                    }),
                    v1_1,
                    std::io::Error::new(std::io::ErrorKind::Other, &*v103),
                    v2,
                )
            }
        }
        pub fn closure20(
            v0_1: string,
            v1_1: u8,
            v2: string,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            Documents::method52(v0_1, v1_1, v2)
        }
        pub fn closure19(
            v0_1: string,
            v1_1: u8,
        ) -> Func1<string, Result<std::path::PathBuf, std::io::Error>> {
            Func1::new({
                let v0_1 = v0_1.clone();
                let v1_1 = v1_1.clone();
                move |v: string| Documents::closure20(v0_1.clone(), v1_1, v)
            })
        }
        pub fn method54(
            v0_1: string,
            v1_1: Func2<u8, string, Result<std::path::PathBuf, std::io::Error>>,
            v2: u8,
            v3: std::io::Error,
        ) -> Result<std::path::PathBuf, std::io::Error> {
            let v4: string = Documents::method45(v0_1.clone());
            let v5: Option<string> = Documents::method29(v0_1.clone());
            let v19: Documents::US5 =
                defaultValue(Documents::US5::US5_1, map(Documents::method6(), v5));
            let v23: string = Documents::method40(v3);
            if (v2) >= 11_u8 {
                let v25: string = sprintf!(
                    "file_system.read_link / path: {} / n: {} / path\': {} / name: {}",
                    v0_1.clone(),
                    v2,
                    v0_1.clone(),
                    v4.clone()
                );
                let v28: std::io::Error = std::io::Error::new(std::io::ErrorKind::Other, &*v25);
                Err(v28)
            } else {
                if let Documents::US5::US5_0(v19_0_0) = &v19 {
                    if (v0_1.clone()) != string("") {
                        let v71: Result<std::path::PathBuf, std::io::Error> =
                            v1_1((v2) + 1_u8, v19_0_0.clone());
                        let v72 = Documents::method39();
                        let v84: Result<std::path::PathBuf, string> = v71.map_err(|x| v72(x));
                        let v87 = Documents::method41();
                        let v88 = Documents::method42();
                        let v89: Documents::US8 = match &v84 {
                            Err(v84_1_0) => v88(v84_1_0.clone()),
                            Ok(v84_0_0) => v87(v84_0_0.clone()),
                        };
                        match &v89 {
                            Documents::US8::US8_0(v89_0_0) => {
                                let v118: string = Documents::method26(
                                    toString(v89_0_0.clone().display()),
                                    v4.clone(),
                                );
                                let v121: &str = &*v118;
                                let v145: std::string::String = String::from(v121);
                                let v169: std::path::PathBuf = std::path::PathBuf::from(v145);
                                Ok(v169)
                            }
                            Documents::US8::US8_1(v89_1_0) => {
                                let v206: string = sprintf!(
                                    "file_system.read_link / error\': {} / error: {} / name: {}",
                                    v89_1_0.clone(),
                                    v23.clone(),
                                    v4.clone()
                                );
                                let v209: std::io::Error =
                                    std::io::Error::new(std::io::ErrorKind::Other, &*v206);
                                Err(v209)
                            }
                        }
                    } else {
                        let v247: string =
                            sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                     v0_1.clone(), v23.clone(), v0_1.clone(),
                                     v4.clone());
                        let v250: std::io::Error =
                            std::io::Error::new(std::io::ErrorKind::Other, &*v247);
                        Err(v250)
                    }
                } else {
                    let v287: string =
                        sprintf!("file_system.read_link / run / The file or directory is not a reparse point. / path: {} / error: {} / path\': {} / name: {}",
                                 v0_1.clone(), v23.clone(), v0_1, v4.clone());
                    let v290: std::io::Error =
                        std::io::Error::new(std::io::ErrorKind::Other, &*v287);
                    Err(v290)
                }
            }
        }
        pub fn method48(v0_1: string, v1_1: u8) -> Result<std::path::PathBuf, std::io::Error> {
            let v29: i32 = Documents::method49(getZero());
            let v31: bool = Documents::method51(Documents::method50(), v29);
            if v31 {
                let v33: () = getZero();
                let v85: std::path::PathBuf = getZero();
                Ok(v85)
            } else {
                let v102: string =
                    sprintf!("file_system.read_link / Fsharp / The file or directory is not a reparse point. / path: {} / result: {} / path\': {} / n: {}",
                             v0_1.clone(), v31, v0_1.clone(), v1_1);
                Documents::method54(
                    v0_1.clone(),
                    Func2::new({
                        let v0_1 = v0_1.clone();
                        move |b0: u8, b1: string| {
                            (Func1::new({
                                let v0_1 = v0_1.clone();
                                move |v: u8| Documents::closure19(v0_1.clone(), v)
                            }))(b0)(b1)
                        }
                    }),
                    v1_1,
                    std::io::Error::new(std::io::ErrorKind::Other, &*v102),
                )
            }
        }
        pub fn method37(v0_1: string) -> Result<std::path::PathBuf, std::io::Error> {
            if Documents::method28(v0_1.clone()) {
                std::fs::read_link(&*v0_1.clone())
            } else {
                Documents::method38(v0_1, 0_u8)
            }
        }
        pub fn closure21(unitVar: (), v0_1: std::path::PathBuf) -> Documents::US9 {
            Documents::US9::US9_0(v0_1)
        }
        pub fn method55() -> Func1<std::path::PathBuf, Documents::US9> {
            Func1::new(move |v: std::path::PathBuf| Documents::closure21((), v))
        }
        pub fn method57(v0_1: string) -> string {
            v0_1
        }
        pub fn method56(v0_1: string, v1_1: string, v2: string) -> string {
            let v5: Result<regex::Regex, regex::Error> = regex::Regex::new(&v0_1);
            let v8: regex::Regex = v5.unwrap();
            let v20: string = Documents::method57(v2);
            let v22: std::borrow::Cow<str> = v8.replace_all(&*v20, &*v1_1);
            let v24: std::string::String = String::from(v22);
            fable_library_rust::String_::fromString(v24)
        }
        pub fn method36(v0_1: string) -> string {
            if (v0_1.clone()) == string("") {
                string("")
            } else {
                let v3: Result<std::path::PathBuf, std::io::Error> =
                    Documents::method37(v0_1.clone());
                let v6: Option<std::path::PathBuf> = v3.ok();
                let v31: Documents::US9 =
                    defaultValue(Documents::US9::US9_1, map(Documents::method55(), v6));
                let v66: string = match &v31 {
                    Documents::US9::US9_0(v31_0_0) => {
                        let v60: string = toString(
                            match &v31 {
                                Documents::US9::US9_0(x) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone()
                            .display(),
                        );
                        if (v60.clone()) == string("") {
                            v0_1.clone()
                        } else {
                            v60
                        }
                    }
                    _ => v0_1.clone(),
                };
                if (v66.clone()) == string("") {
                    string("")
                } else {
                    let v71: string =
                        Documents::method56(string("^\\\\\\\\\\?\\\\"), string(""), v66);
                    replace(
                        concat(new_array(&[
                            toLower(ofChar(getCharAt(v71.clone(), 0_i32))),
                            getSlice(v71, Some(1_i32), None::<i32>),
                        ])),
                        string("\\"),
                        string("/"),
                    )
                }
            }
        }
        pub fn method58(v0_1: i32, v1_1: LrcPtr<Documents::Mut5>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method59(v0_1: i32, v1_1: LrcPtr<Documents::Mut6>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method60() -> char {
            std::path::MAIN_SEPARATOR
        }
        pub fn method61(v0_1: string) -> string {
            v0_1
        }
        pub fn method35(v0_1: string) -> string {
            let v4: &str = &*v0_1.clone();
            let v28: std::string::String = String::from(v4);
            let v52: std::path::PathBuf = std::path::PathBuf::from(v28);
            if (v52.exists()) == false {
                let v77: string = Documents::method34();
                let v81: Array<string> = split(
                    Documents::method36(Documents::method26(v77.clone(), v0_1.clone())),
                    string("/"),
                    -1_i32,
                    0_i32,
                );
                let v85: i32 = get_Count(v81.clone());
                let v86: LrcPtr<Documents::Mut5> = LrcPtr::new(Documents::Mut5 {
                    l0: MutCell::new(0_i32),
                    l1: MutCell::new(0_i32),
                    l2: MutCell::new(new_empty::<string>()),
                });
                while Documents::method58(v85, v86.clone()) {
                    let v88: i32 = v86.l0.get().clone();
                    let v91: i32 = ((v88.wrapping_neg()) + (v85)) - 1_i32;
                    let matchValue: i32 = v86.l1.get().clone();
                    let v93: Array<string> = v86.l2.get().clone();
                    let v92: i32 = matchValue;
                    let v94: string = v81[v91].clone();
                    let patternInput_1: (i32, Array<string>) = if string("..") == (v94.clone()) {
                        ((v92) + 1_i32, v93.clone())
                    } else {
                        if string(".") == (v94.clone()) {
                            (v92, v93.clone())
                        } else {
                            if 0_i32 == (v92) {
                                if endsWith3(v94.clone(), string(":"), false) {
                                    let v104: Array<string> = new_array(&[sprintf!(
                                        "{}:",
                                        getCharAt(v77.clone(), 0_i32)
                                    )]);
                                    let v105: i32 = get_Count(v104.clone());
                                    let v107: i32 = (v105) + (get_Count(v93.clone()));
                                    let v108: Array<string> = new_init(&string(""), v107);
                                    let v109: LrcPtr<Documents::Mut6> =
                                        LrcPtr::new(Documents::Mut6 {
                                            l0: MutCell::new(0_i32),
                                        });
                                    while Documents::method59(v107, v109.clone()) {
                                        let v111: i32 = v109.l0.get().clone();
                                        let v116: string = if (v111) < (v105) {
                                            v104[v111].clone()
                                        } else {
                                            let v114: i32 = (v111) - (v105);
                                            v93[v114].clone()
                                        };
                                        v108.get_mut()[v111 as usize] = v116;
                                        {
                                            let v117: i32 = (v111) + 1_i32;
                                            v109.l0.set(v117);
                                            ()
                                        }
                                    }
                                    (0_i32, v108.clone())
                                } else {
                                    let v118: Array<string> = new_array(&[v94]);
                                    let v119: i32 = get_Count(v118.clone());
                                    let v121: i32 = (v119) + (get_Count(v93.clone()));
                                    let v122: Array<string> = new_init(&string(""), v121);
                                    let v123: LrcPtr<Documents::Mut6> =
                                        LrcPtr::new(Documents::Mut6 {
                                            l0: MutCell::new(0_i32),
                                        });
                                    while Documents::method59(v121, v123.clone()) {
                                        let v125: i32 = v123.l0.get().clone();
                                        let v130: string = if (v125) < (v119) {
                                            v118[v125].clone()
                                        } else {
                                            let v128: i32 = (v125) - (v119);
                                            v93[v128].clone()
                                        };
                                        v122.get_mut()[v125 as usize] = v130;
                                        {
                                            let v131: i32 = (v125) + 1_i32;
                                            v123.l0.set(v131);
                                            ()
                                        }
                                    }
                                    (0_i32, v122.clone())
                                }
                            } else {
                                ((v92) - 1_i32, v93.clone())
                            }
                        }
                    };
                    let v141: i32 = (v88) + 1_i32;
                    v86.l0.set(v141);
                    v86.l1.set(patternInput_1.0.clone());
                    v86.l2.set(patternInput_1.1.clone());
                    ()
                }
                {
                    let matchValue_2: i32 = v86.l1.get().clone();
                    let v143: Array<string> = v86.l2.get().clone();
                    let _v144: LrcPtr<dyn IEnumerable_1<string>> = delay(Func0::new({
                        let v143 = v143.clone();
                        move || {
                            map_1(
                                Func1::new({
                                    let v143 = v143.clone();
                                    move |i: i32| v143[i].clone()
                                }),
                                rangeNumeric(0_i32, 1_i32, (get_Count(v143.clone())) - 1_i32),
                            )
                        }
                    }));
                    let v155: string = ofChar(Documents::method60());
                    join(
                        if (v155.clone()) == string("\n") {
                            Documents::method61(v155.clone())
                        } else {
                            v155
                        },
                        toArray_1(_v144),
                    )
                }
            } else {
                let v167: Result<std::path::PathBuf, std::io::Error> =
                    std::fs::canonicalize(&*v0_1);
                let v170: std::path::PathBuf = v167.unwrap();
                let v184: std::path::Display = v170.display();
                let v208: std::string::String = format!("{}", v184);
                fable_library_rust::String_::fromString(v208)
            }
        }
        pub fn method62() -> string {
            let v6: string = Documents::method13(getCharAt(toLower(string("Debug")), 0_i32));
            let v9: &str = inline_colorization::color_bright_blue;
            let v12: &str = &*v6;
            let v35: &str = inline_colorization::color_reset;
            let v37: std::string::String = format!("{}{}{}", v9, v12, v35);
            fable_library_rust::String_::fromString(v37)
        }
        pub fn method64(v0_1: string, v1_1: string, v2: string, v3: string) -> string {
            let v5: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v12: () = {
                Documents::closure8(v5.clone(), string("{ "), ());
                ()
            };
            let v21: () = {
                Documents::closure8(v5.clone(), string("source_dir"), ());
                ()
            };
            let v30: () = {
                Documents::closure8(v5.clone(), string(" = "), ());
                ()
            };
            let v38: () = {
                Documents::closure8(v5.clone(), v0_1, ());
                ()
            };
            let v47: () = {
                Documents::closure8(v5.clone(), string("; "), ());
                ()
            };
            let v56: () = {
                Documents::closure8(v5.clone(), string("dist_dir"), ());
                ()
            };
            let v64: () = {
                Documents::closure8(v5.clone(), string(" = "), ());
                ()
            };
            let v72: () = {
                Documents::closure8(v5.clone(), v1_1, ());
                ()
            };
            let v80: () = {
                Documents::closure8(v5.clone(), string("; "), ());
                ()
            };
            let v89: () = {
                Documents::closure8(v5.clone(), string("cache_dir"), ());
                ()
            };
            let v97: () = {
                Documents::closure8(v5.clone(), string(" = "), ());
                ()
            };
            let v105: () = {
                Documents::closure8(v5.clone(), v2, ());
                ()
            };
            let v113: () = {
                Documents::closure8(v5.clone(), string("; "), ());
                ()
            };
            let v122: () = {
                Documents::closure8(v5.clone(), string("hangul_spec"), ());
                ()
            };
            let v130: () = {
                Documents::closure8(v5.clone(), string(" = "), ());
                ()
            };
            let v138: () = {
                Documents::closure8(v5.clone(), v3, ());
                ()
            };
            let v147: () = {
                Documents::closure8(v5.clone(), string(" }"), ());
                ()
            };
            v5.l0.get().clone()
        }
        pub fn method63(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
            v10: string,
            v11: string,
        ) -> string {
            let v12: string = Documents::method64(v8, v9, v10, v11);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.run"),
                v12
            ))
        }
        pub fn closure22(v0_1: string, v1_1: string, v2: string, v3: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_1) {
                let v8: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v27: Option<i64> = patternInput.5.clone();
                let v26: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v25: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v24: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v23: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v22: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method63(
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    v26.clone(),
                    v27.clone(),
                    Documents::method8(v22, v23, v24, v25, v26, v27),
                    Documents::method62(),
                    v1_1,
                    v2,
                    v3,
                    v0_1,
                ))
            };
        }
        pub fn closure24(unitVar: (), v0_1: std::io::Error) -> std::string::String {
            format!("{}", v0_1)
        }
        pub fn method65() -> Func1<std::io::Error, std::string::String> {
            Func1::new(move |v: std::io::Error| Documents::closure24((), v))
        }
        pub fn closure25(unitVar: (), v0_1: std::fs::FileType) -> Documents::US10 {
            Documents::US10::US10_0(v0_1)
        }
        pub fn method66() -> Func1<std::fs::FileType, Documents::US10> {
            Func1::new(move |v: std::fs::FileType| Documents::closure25((), v))
        }
        pub fn closure26(unitVar: (), v0_1: std::string::String) -> Documents::US10 {
            Documents::US10::US10_1(v0_1)
        }
        pub fn method67() -> Func1<std::string::String, Documents::US10> {
            Func1::new(move |v: std::string::String| Documents::closure26((), v))
        }
        pub fn method68(v0_1: Documents::US11) -> Documents::US11 {
            v0_1
        }
        pub fn method69(v0_1: async_walkdir::Filtering) -> async_walkdir::Filtering {
            v0_1
        }
        pub fn closure23(
            unitVar: (),
            v0_1: async_walkdir::DirEntry,
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = async_walkdir::Filtering> + Send>>
        {
            let v2: bool = true;
            let __future_init = Box::pin(async {
                //;
                let v4: bool = true;
                let __future_init = Box::pin(async move {
                    //;
                    let v6: async_walkdir::DirEntry = v0_1.clone();
                    let v8: std::pin::Pin<
                        Box<
                            dyn std::future::Future<
                                    Output = Result<std::fs::FileType, std::io::Error>,
                                > + Send,
                        >,
                    > = Box::pin(async_walkdir::DirEntry::file_type(&v6));
                    let v10: Result<std::fs::FileType, std::io::Error> = v8.await;
                    let v11 = Documents::method65();
                    let v23: Result<std::fs::FileType, std::string::String> =
                        v10.map_err(|x| v11(x));
                    let v26 = Documents::method66();
                    let v27 = Documents::method67();
                    let v28: Documents::US10 = match &v23 {
                        Err(v23_1_0) => v27(v23_1_0.clone()),
                        Ok(v23_0_0) => v26(v23_0_0.clone()),
                    };
                    let v165: Documents::US11 =
                        Documents::method68(if let Documents::US10::US10_0(v28_0_0) = &v28 {
                            if std::fs::FileType::is_dir(&v28_0_0.clone()) {
                                Documents::US11::US11_0
                            } else {
                                let v34: std::path::PathBuf =
                                    async_walkdir::DirEntry::path(&v0_1.clone());
                                let v37: std::path::Display = v34.display();
                                let v61: std::string::String = format!("{}", v37);
                                let v84: string = fable_library_rust::String_::fromString(v61);
                                if if (endsWith3(v84.clone(), string(".md"), false)) == false {
                                    true
                                } else {
                                    endsWith3(v84, string(".hangul.md"), false)
                                } {
                                    Documents::US11::US11_0
                                } else {
                                    Documents::US11::US11_2
                                }
                            }
                        } else {
                            let v100: std::path::PathBuf = async_walkdir::DirEntry::path(&v0_1);
                            let v103: std::path::Display = v100.display();
                            let v127: std::string::String = format!("{}", v103);
                            let v150: string = fable_library_rust::String_::fromString(v127);
                            if if (endsWith3(v150.clone(), string(".md"), false)) == false {
                                true
                            } else {
                                endsWith3(v150, string(".hangul.md"), false)
                            } {
                                Documents::US11::US11_0
                            } else {
                                Documents::US11::US11_2
                            }
                        });
                    let v168: string = string("}");
                    let v172: bool = true;
                    let v169 = v165;
                    let v184: string = append(
                        (append(
                            (append((append(string("true; v169 "), (v168))), string("); "))),
                            string(""),
                        )),
                        string(" // rust.fix_closure\'"),
                    );
                    let v185: bool = true;
                    v169
                }); // rust.fix_closure';
                let v187 = __future_init;
                let v189: std::pin::Pin<
                    Box<dyn std::future::Future<Output = Documents::US11> + Send>,
                > = v187;
                let v191: Documents::US11 = v189.await;
                let v201: async_walkdir::Filtering = Documents::method69(match &v191 {
                    Documents::US11::US11_0 => async_walkdir::Filtering::Ignore,
                    Documents::US11::US11_1 => async_walkdir::Filtering::IgnoreDir,
                    _ => async_walkdir::Filtering::Continue,
                });
                let v202: string = string("}");
                let v206: bool = true;
                let v203 = v201;
                let v218: string = append(
                    (append(
                        (append((append(string("true; v203 "), (v202))), string("); "))),
                        string(""),
                    )),
                    string(" // rust.fix_closure\'"),
                );
                let v219: bool = true;
                v203
            }); // rust.fix_closure';
            let v221 = __future_init;
            v221
        }
        pub fn closure28(unitVar: (), v0_1: async_walkdir::Error) -> std::string::String {
            format!("{}", v0_1)
        }
        pub fn method71() -> Func1<async_walkdir::Error, std::string::String> {
            Func1::new(move |v: async_walkdir::Error| Documents::closure28((), v))
        }
        pub fn closure29(unitVar: (), v0_1: async_walkdir::DirEntry) -> Documents::US12 {
            Documents::US12::US12_0(v0_1)
        }
        pub fn method72() -> Func1<async_walkdir::DirEntry, Documents::US12> {
            Func1::new(move |v: async_walkdir::DirEntry| Documents::closure29((), v))
        }
        pub fn closure30(unitVar: (), v0_1: std::string::String) -> Documents::US12 {
            Documents::US12::US12_1(v0_1)
        }
        pub fn method73() -> Func1<std::string::String, Documents::US12> {
            Func1::new(move |v: std::string::String| Documents::closure30((), v))
        }
        pub fn method74() -> string {
            let v6: string = Documents::method13(getCharAt(toLower(string("Critical")), 0_i32));
            let v9: &str = inline_colorization::color_bright_red;
            let v12: &str = &*v6;
            let v35: &str = inline_colorization::color_reset;
            let v37: std::string::String = format!("{}{}{}", v9, v12, v35);
            fable_library_rust::String_::fromString(v37)
        }
        pub fn method76(v0_1: std::string::String) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("error"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v32: std::string::String = format!("{:#?}", v0_1);
            let v65: () = {
                Documents::closure8(v2.clone(), fable_library_rust::String_::fromString(v32), ());
                ()
            };
            let v74: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method75(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
        ) -> string {
            let v9: string = Documents::method76(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.run / stream_filter_map"),
                v9
            ))
        }
        pub fn closure31(v0_1: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_4) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method75(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method74(),
                    v0_1,
                ))
            };
        }
        pub fn closure27(
            unitVar: (),
            v0_1: Result<async_walkdir::DirEntry, async_walkdir::Error>,
        ) -> Option<string> {
            let v1_1 = Documents::method71();
            let v13: Result<async_walkdir::DirEntry, std::string::String> =
                v0_1.map_err(|x| v1_1(x));
            let v16 = Documents::method72();
            let v17 = Documents::method73();
            let v18: Documents::US12 = match &v13 {
                Err(v13_1_0) => v17(v13_1_0.clone()),
                Ok(v13_0_0) => v16(v13_0_0.clone()),
            };
            let v118: Documents::US5 = match &v18 {
                Documents::US12::US12_0(v18_0_0) => {
                    let v21: std::path::PathBuf = async_walkdir::DirEntry::path(&v18_0_0.clone());
                    let v24: std::path::Display = v21.display();
                    let v48: std::string::String = format!("{}", v24);
                    Documents::US5::US5_0(fable_library_rust::String_::fromString(v48))
                }
                Documents::US12::US12_1(v18_1_0) => {
                    let v76: () = {
                        Documents::closure31(v18_1_0.clone(), ());
                        ()
                    };
                    Documents::US5::US5_1
                }
            };
            match &v118 {
                Documents::US5::US5_0(v118_0_0) => Some(
                    match &v118 {
                        Documents::US5::US5_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                ),
                _ => None::<string>,
            }
        }
        pub fn method70(
        ) -> Func1<Result<async_walkdir::DirEntry, async_walkdir::Error>, Option<string>> {
            Func1::new(
                move |v: Result<async_walkdir::DirEntry, async_walkdir::Error>| {
                    Documents::closure27((), v)
                },
            )
        }
        pub fn method78(v0_1: usize) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("files_len"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v32: std::string::String = format!("{:#?}", v0_1);
            let v65: () = {
                Documents::closure8(v2.clone(), fable_library_rust::String_::fromString(v32), ());
                ()
            };
            let v74: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method77(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: usize,
        ) -> string {
            let v9: string = Documents::method78(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.run"),
                v9
            ))
        }
        pub fn closure32(v0_1: Vec<string>, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_1) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method77(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method62(),
                    v0_1.len(),
                ))
            };
        }
        pub fn method79() -> string {
            string("")
        }
        pub fn method80(v0_1: string) -> string {
            Documents::method36(Documents::method35(v0_1))
        }
        pub fn method82(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
        ) -> string {
            v0_1
        }
        pub fn method84() -> string {
            string("")
        }
        pub fn closure36(v0_1: char, v1_1: LrcPtr<Documents::UH0>) -> LrcPtr<Documents::UH0> {
            LrcPtr::new(Documents::UH0::UH0_1(v0_1, v1_1))
        }
        pub fn closure35(
            unitVar: (),
            v0_1: char,
        ) -> Func1<LrcPtr<Documents::UH0>, LrcPtr<Documents::UH0>> {
            Func1::new({
                let v0_1 = v0_1.clone();
                move |v: LrcPtr<Documents::UH0>| Documents::closure36(v0_1, v)
            })
        }
        pub fn method85() -> Func1<char, Func1<LrcPtr<Documents::UH0>, LrcPtr<Documents::UH0>>> {
            Func1::new(move |v: char| Documents::closure35((), v))
        }
        pub fn method86(
            v0_1: LrcPtr<Documents::UH0>,
            v1_1: LrcPtr<StringBuilder>,
            v2: i32,
            v3: i32,
        ) -> (LrcPtr<StringBuilder>, i32, i32) {
            let v0_1: MutCell<LrcPtr<Documents::UH0>> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v1_1.clone());
            let v2: MutCell<i32> = MutCell::new(v2);
            let v3: MutCell<i32> = MutCell::new(v3);
            '_method86: loop {
                break '_method86 (match v0_1.get().clone().as_ref() {
                    Documents::UH0::UH0_0 => {
                        (v1_1.get().clone(), v2.get().clone(), v3.get().clone())
                    }
                    Documents::UH0::UH0_1(v0_1_1_0, v0_1_1_1) => {
                        let v4: char = match v0_1.get().clone().as_ref() {
                            Documents::UH0::UH0_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        };
                        let v6: bool = '\n' == (v4);
                        let patternInput: (i32, i32) = if v6 {
                            ((v2.get().clone()) + 1_i32, 1_i32)
                        } else {
                            (v2.get().clone(), (v3.get().clone()) + 1_i32)
                        };
                        {
                            let v0_1_temp: LrcPtr<Documents::UH0> =
                                match v0_1.get().clone().as_ref() {
                                    Documents::UH0::UH0_1(_, x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone();
                            let v1_1_temp: LrcPtr<StringBuilder> = if v6 {
                                let v12: LrcPtr<StringBuilder> = v1_1.get().clone().Clear();
                                v1_1.get().clone()
                            } else {
                                let v21: LrcPtr<StringBuilder> = {
                                    let value: string = ofChar(v4);
                                    v1_1.get().clone().Append_Z721C83C5(value)
                                };
                                v1_1.get().clone()
                            };
                            let v2_temp: i32 = patternInput.0.clone();
                            let v3_temp: i32 = patternInput.1.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            continue '_method86;
                        }
                    }
                });
            }
        }
        pub fn closure34(
            unitVar: (),
            _arg: (string, LrcPtr<StringBuilder>, i32, i32),
        ) -> Documents::US14 {
            let v3: i32 = _arg.3.clone();
            let v2: i32 = _arg.2.clone();
            let v1_1: LrcPtr<StringBuilder> = _arg.1.clone();
            let v0_1: string = _arg.0.clone();
            if string("") == (v0_1.clone()) {
                Documents::US14::US14_1(sprintf!(
                    "parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                    '\"',
                    (v1_1.clone(), v2, v3)
                ))
            } else {
                let v9: char = getCharAt(v0_1.clone(), 0_i32);
                if (v9) == '\"' {
                    let v30: string = getSlice(
                        v0_1.clone(),
                        Some(1_i32),
                        Some((length(v0_1.clone())) - 1_i32),
                    );
                    let v35: string = ofChar(v9);
                    let v38: i32 = length(v35.clone());
                    let v39: Array<char> = new_init(&'\u{0000}', v38);
                    let v40: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                        l0: MutCell::new(0_i32),
                    });
                    while Documents::method59(v38, v40.clone()) {
                        let v42: i32 = v40.l0.get().clone();
                        let v43: char = getCharAt(v35.clone(), v42);
                        v39.get_mut()[v42 as usize] = v43;
                        {
                            let v44: i32 = (v42) + 1_i32;
                            v40.l0.set(v44);
                            ()
                        }
                    }
                    {
                        let v45: List<char> = ofArray(v39.clone());
                        let patternInput: (LrcPtr<StringBuilder>, i32, i32) = Documents::method86(
                            foldBack(
                                Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                    (Documents::method85())(b0)(b1)
                                }),
                                v45,
                                LrcPtr::new(Documents::UH0::UH0_0),
                            ),
                            v1_1.clone(),
                            v2,
                            v3,
                        );
                        Documents::US14::US14_0(
                            v9,
                            v30,
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            patternInput.2.clone(),
                        )
                    }
                } else {
                    let v83: i32 = (indexOf(v0_1.clone(), string("\n"))) - 1_i32;
                    Documents::US14::US14_1(concat(new_array(&[
                        sprintf!(
                            "parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                            '\"',
                            v2,
                            v3,
                            v1_1,
                            getSlice(
                                v0_1.clone(),
                                Some(0_i32),
                                Some(
                                    (if -2_i32 == (v83) {
                                        (length(v0_1)) + 1_i32
                                    } else {
                                        (v83) + 1_i32
                                    }) - 1_i32
                                )
                            )
                        ),
                        string("\n"),
                        append((replicate((v3) - 1_i32, string(" "))), string("^")),
                        string("\n"),
                    ])))
                }
            }
        }
        pub fn closure37(
            unitVar: (),
            _arg: (string, LrcPtr<StringBuilder>, i32, i32),
        ) -> Documents::US14 {
            let v3: i32 = _arg.3.clone();
            let v2: i32 = _arg.2.clone();
            let v1_1: LrcPtr<StringBuilder> = _arg.1.clone();
            let v0_1: string = _arg.0.clone();
            if string("") == (v0_1.clone()) {
                Documents::US14::US14_1(sprintf!(
                    "parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                    '\'',
                    (v1_1.clone(), v2, v3)
                ))
            } else {
                let v9: char = getCharAt(v0_1.clone(), 0_i32);
                if (v9) == '\'' {
                    let v30: string = getSlice(
                        v0_1.clone(),
                        Some(1_i32),
                        Some((length(v0_1.clone())) - 1_i32),
                    );
                    let v35: string = ofChar(v9);
                    let v38: i32 = length(v35.clone());
                    let v39: Array<char> = new_init(&'\u{0000}', v38);
                    let v40: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                        l0: MutCell::new(0_i32),
                    });
                    while Documents::method59(v38, v40.clone()) {
                        let v42: i32 = v40.l0.get().clone();
                        let v43: char = getCharAt(v35.clone(), v42);
                        v39.get_mut()[v42 as usize] = v43;
                        {
                            let v44: i32 = (v42) + 1_i32;
                            v40.l0.set(v44);
                            ()
                        }
                    }
                    {
                        let v45: List<char> = ofArray(v39.clone());
                        let patternInput: (LrcPtr<StringBuilder>, i32, i32) = Documents::method86(
                            foldBack(
                                Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                    (Documents::method85())(b0)(b1)
                                }),
                                v45,
                                LrcPtr::new(Documents::UH0::UH0_0),
                            ),
                            v1_1.clone(),
                            v2,
                            v3,
                        );
                        Documents::US14::US14_0(
                            v9,
                            v30,
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            patternInput.2.clone(),
                        )
                    }
                } else {
                    let v83: i32 = (indexOf(v0_1.clone(), string("\n"))) - 1_i32;
                    Documents::US14::US14_1(concat(new_array(&[
                        sprintf!(
                            "parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                            '\'',
                            v2,
                            v3,
                            v1_1,
                            getSlice(
                                v0_1.clone(),
                                Some(0_i32),
                                Some(
                                    (if -2_i32 == (v83) {
                                        (length(v0_1)) + 1_i32
                                    } else {
                                        (v83) + 1_i32
                                    }) - 1_i32
                                )
                            )
                        ),
                        string("\n"),
                        append((replicate((v3) - 1_i32, string(" "))), string("^")),
                        string("\n"),
                    ])))
                }
            }
        }
        pub fn method87(
            v0_1: string,
            v1_1: LrcPtr<StringBuilder>,
            v2: LrcPtr<Documents::UH1>,
        ) -> Documents::US14 {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<Documents::UH1>> = MutCell::new(v2.clone());
            '_method87: loop {
                break '_method87 (match v2.get().clone().as_ref() {
                    Documents::UH1::UH1_0 => {
                        Documents::US14::US14_1(string("parsing.choice / no parsers succeeded"))
                    }
                    Documents::UH1::UH1_1(v2_1_0, v2_1_1) => {
                        let v7: Documents::US14 = (match v2.get().clone().as_ref() {
                            Documents::UH1::UH1_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        })((
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            1_i32,
                            1_i32,
                        ));
                        match &v7 {
                            Documents::US14::US14_0(v7_0_0, v7_0_1, v7_0_2, v7_0_3, v7_0_4) => {
                                v7.clone()
                            }
                            _ => {
                                let v0_1_temp: string = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<StringBuilder> = v1_1.get().clone();
                                let v2_temp: LrcPtr<Documents::UH1> =
                                    match v2.get().clone().as_ref() {
                                        Documents::UH1::UH1_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                v2.set(v2_temp);
                                continue '_method87;
                            }
                        }
                    }
                });
            }
        }
        pub fn method88(v0_1: char, v1_1: i64) -> bool {
            let v0_1: MutCell<char> = MutCell::new(v0_1);
            let v1_1: MutCell<i64> = MutCell::new(v1_1);
            '_method88: loop {
                break '_method88 (if (v1_1.get().clone()) >= 2_i64 {
                    false
                } else {
                    let v11: Documents::US16 = if (v1_1.get().clone()) == 0_i64 {
                        Documents::US16::US16_0('\"')
                    } else {
                        let v5: i64 = (v1_1.get().clone()) - 1_i64;
                        if (v5) == 0_i64 {
                            Documents::US16::US16_0('\'')
                        } else {
                            let v8: i64 = (v5) - 1_i64;
                            Documents::US16::US16_1
                        }
                    };
                    if (v0_1.get().clone())
                        == (match &v11 {
                            Documents::US16::US16_0(v11_0_0) => match &v11 {
                                Documents::US16::US16_0(x) => x.clone(),
                                _ => unreachable!(),
                            },
                            _ => panic!("{}", string("Option does not have a value."),),
                        })
                    {
                        true
                    } else {
                        let v0_1_temp: char = v0_1.get().clone();
                        let v1_1_temp: i64 = (v1_1.get().clone()) + 1_i64;
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method88;
                    }
                });
            }
        }
        pub fn method89(
            v0_1: string,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> (string, string, LrcPtr<StringBuilder>, i32, i32) {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method89: loop {
                break '_method89 ({
                    let v109: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\"', '\'']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v21: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method88(v21, 0_i64)) == false {
                            let v44: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v49: string = ofChar(v21);
                            let v52: i32 = length(v49.clone());
                            let v53: Array<char> = new_init(&'\u{0000}', v52);
                            let v54: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v52, v54.clone()) {
                                let v56: i32 = v54.l0.get().clone();
                                let v57: char = getCharAt(v49.clone(), v56);
                                v53.get_mut()[v56 as usize] = v57;
                                {
                                    let v58: i32 = (v56) + 1_i32;
                                    v54.l0.set(v58);
                                    ()
                                }
                            }
                            {
                                let v59: List<char> = ofArray(v53.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v59,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v21,
                                    v44,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v21,
                                toArray(ofArray(new_array(&['\"', '\'']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    let v121: Documents::US14 = match &v109 {
                        Documents::US14::US14_0(
                            v109_0_0,
                            v109_0_1,
                            v109_0_2,
                            v109_0_3,
                            v109_0_4,
                        ) => {
                            let v110: char = v109_0_0.clone();
                            Documents::US14::US14_0(
                                if '\\' == (v110) { '/' } else { v110 },
                                v109_0_1.clone(),
                                v109_0_2.clone(),
                                v109_0_3.clone(),
                                v109_0_4.clone(),
                            )
                        }
                        Documents::US14::US14_1(v109_1_0) => {
                            Documents::US14::US14_1(v109_1_0.clone())
                        }
                    };
                    match &v121 {
                        Documents::US14::US14_0(
                            v121_0_0,
                            v121_0_1,
                            v121_0_2,
                            v121_0_3,
                            v121_0_4,
                        ) => {
                            let v0_1_temp: string =
                                append((v0_1.get().clone()), (ofChar(v121_0_0.clone())));
                            let v1_1_temp: string = v121_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v121_0_2.clone();
                            let v3_temp: i32 = v121_0_3.clone();
                            let v4_temp: i32 = v121_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method89;
                        }
                        _ => (
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method90(
            v0_1: string,
            v1_1: LrcPtr<StringBuilder>,
            v2: i32,
            v3: i32,
            v4: LrcPtr<Documents::UH1>,
        ) -> Documents::US14 {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v1_1.clone());
            let v2: MutCell<i32> = MutCell::new(v2);
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<LrcPtr<Documents::UH1>> = MutCell::new(v4.clone());
            '_method90: loop {
                break '_method90 (match v4.get().clone().as_ref() {
                    Documents::UH1::UH1_0 => {
                        Documents::US14::US14_1(string("parsing.choice / no parsers succeeded"))
                    }
                    Documents::UH1::UH1_1(v4_1_0, v4_1_1) => {
                        let v9: Documents::US14 = (match v4.get().clone().as_ref() {
                            Documents::UH1::UH1_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        })((
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                        ));
                        match &v9 {
                            Documents::US14::US14_0(v9_0_0, v9_0_1, v9_0_2, v9_0_3, v9_0_4) => {
                                v9.clone()
                            }
                            _ => {
                                let v0_1_temp: string = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<StringBuilder> = v1_1.get().clone();
                                let v2_temp: i32 = v2.get().clone();
                                let v3_temp: i32 = v3.get().clone();
                                let v4_temp: LrcPtr<Documents::UH1> =
                                    match v4.get().clone().as_ref() {
                                        Documents::UH1::UH1_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                v2.set(v2_temp);
                                v3.set(v3_temp);
                                v4.set(v4_temp);
                                continue '_method90;
                            }
                        }
                    }
                });
            }
        }
        pub fn method91(v0_1: char, v1_1: i64) -> bool {
            let v0_1: MutCell<char> = MutCell::new(v0_1);
            let v1_1: MutCell<i64> = MutCell::new(v1_1);
            '_method91: loop {
                break '_method91 (if (v1_1.get().clone()) >= 3_i64 {
                    false
                } else {
                    let v15: Documents::US16 = if (v1_1.get().clone()) == 0_i64 {
                        Documents::US16::US16_0('\"')
                    } else {
                        let v5: i64 = (v1_1.get().clone()) - 1_i64;
                        if (v5) == 0_i64 {
                            Documents::US16::US16_0('\'')
                        } else {
                            let v8: i64 = (v5) - 1_i64;
                            if (v8) == 0_i64 {
                                Documents::US16::US16_0(' ')
                            } else {
                                let v11: i64 = (v8) - 1_i64;
                                Documents::US16::US16_1
                            }
                        }
                    };
                    if (v0_1.get().clone())
                        == (match &v15 {
                            Documents::US16::US16_0(v15_0_0) => match &v15 {
                                Documents::US16::US16_0(x) => x.clone(),
                                _ => unreachable!(),
                            },
                            _ => panic!("{}", string("Option does not have a value."),),
                        })
                    {
                        true
                    } else {
                        let v0_1_temp: char = v0_1.get().clone();
                        let v1_1_temp: i64 = (v1_1.get().clone()) + 1_i64;
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method91;
                    }
                });
            }
        }
        pub fn method92(
            v0_1: string,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> (string, string, LrcPtr<StringBuilder>, i32, i32) {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method92: loop {
                break '_method92 ({
                    let v115: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\"', '\'', ' ']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v24: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method91(v24, 0_i64)) == false {
                            let v47: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v52: string = ofChar(v24);
                            let v55: i32 = length(v52.clone());
                            let v56: Array<char> = new_init(&'\u{0000}', v55);
                            let v57: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v55, v57.clone()) {
                                let v59: i32 = v57.l0.get().clone();
                                let v60: char = getCharAt(v52.clone(), v59);
                                v56.get_mut()[v59 as usize] = v60;
                                {
                                    let v61: i32 = (v59) + 1_i32;
                                    v57.l0.set(v61);
                                    ()
                                }
                            }
                            {
                                let v62: List<char> = ofArray(v56.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v62,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v24,
                                    v47,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v24,
                                toArray(ofArray(new_array(&['\"', '\'', ' ']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    let v127: Documents::US14 = match &v115 {
                        Documents::US14::US14_0(
                            v115_0_0,
                            v115_0_1,
                            v115_0_2,
                            v115_0_3,
                            v115_0_4,
                        ) => {
                            let v116: char = v115_0_0.clone();
                            Documents::US14::US14_0(
                                if '\\' == (v116) { '/' } else { v116 },
                                v115_0_1.clone(),
                                v115_0_2.clone(),
                                v115_0_3.clone(),
                                v115_0_4.clone(),
                            )
                        }
                        Documents::US14::US14_1(v115_1_0) => {
                            Documents::US14::US14_1(v115_1_0.clone())
                        }
                    };
                    match &v127 {
                        Documents::US14::US14_0(
                            v127_0_0,
                            v127_0_1,
                            v127_0_2,
                            v127_0_3,
                            v127_0_4,
                        ) => {
                            let v0_1_temp: string =
                                append((v0_1.get().clone()), (ofChar(v127_0_0.clone())));
                            let v1_1_temp: string = v127_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v127_0_2.clone();
                            let v3_temp: i32 = v127_0_3.clone();
                            let v4_temp: i32 = v127_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method92;
                        }
                        _ => (
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method93(v0_1: string, v1_1: i32) -> i32 {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<i32> = MutCell::new(v1_1);
            '_method93: loop {
                break '_method93 (if (v1_1.get().clone()) >= (length(v0_1.get().clone())) {
                    v1_1.get().clone()
                } else {
                    if ' ' == (getCharAt(v0_1.get().clone(), v1_1.get().clone())) {
                        let v0_1_temp: string = v0_1.get().clone();
                        let v1_1_temp: i32 = (v1_1.get().clone()) + 1_i32;
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method93;
                    } else {
                        v1_1.get().clone()
                    }
                });
            }
        }
        pub fn method94(
            v0_1: string,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> (string, string, LrcPtr<StringBuilder>, i32, i32) {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method94: loop {
                break '_method94 ({
                    let v79: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.any_char / unexpected end of input / s: {:?}",
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v10: char = getCharAt(v1_1.get().clone(), 0_i32);
                        let v30: string = getSlice(
                            v1_1.get().clone(),
                            Some(1_i32),
                            Some((length(v1_1.get().clone())) - 1_i32),
                        );
                        let v35: string = ofChar(v10);
                        let v38: i32 = length(v35.clone());
                        let v39: Array<char> = new_init(&'\u{0000}', v38);
                        let v40: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                            l0: MutCell::new(0_i32),
                        });
                        while Documents::method59(v38, v40.clone()) {
                            let v42: i32 = v40.l0.get().clone();
                            let v43: char = getCharAt(v35.clone(), v42);
                            v39.get_mut()[v42 as usize] = v43;
                            {
                                let v44: i32 = (v42) + 1_i32;
                                v40.l0.set(v44);
                                ()
                            }
                        }
                        {
                            let v45: List<char> = ofArray(v39.clone());
                            let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                Documents::method86(
                                    foldBack(
                                        Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                            (Documents::method85())(b0)(b1)
                                        }),
                                        v45,
                                        LrcPtr::new(Documents::UH0::UH0_0),
                                    ),
                                    v2.get().clone(),
                                    v3.get().clone(),
                                    v4.get().clone(),
                                );
                            Documents::US14::US14_0(
                                v10,
                                v30,
                                patternInput.0.clone(),
                                patternInput.1.clone(),
                                patternInput.2.clone(),
                            )
                        }
                    };
                    match &v79 {
                        Documents::US14::US14_0(v79_0_0, v79_0_1, v79_0_2, v79_0_3, v79_0_4) => {
                            let v0_1_temp: string =
                                append((v0_1.get().clone()), (ofChar(v79_0_0.clone())));
                            let v1_1_temp: string = v79_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v79_0_2.clone();
                            let v3_temp: i32 = v79_0_3.clone();
                            let v4_temp: i32 = v79_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method94;
                        }
                        _ => (
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method83(v0_1: string) -> Documents::US13 {
            let _v0: MutCell<Option<Option<string>>> = MutCell::new(None::<Option<string>>);
            _v0.set(Some(Some(v0_1)));
            {
                let v7: string = defaultValue(
                    string(""),
                    match &_v0.get().clone() {
                        None => panic!("{}", string("optionm\'.of_obj / _v0=None"),),
                        Some(_v0_0_0) => _v0_0_0.clone(),
                    },
                );
                let v12: LrcPtr<StringBuilder> =
                    StringBuilder::_ctor__Z721C83C5(Documents::method84());
                fn v15(arg10_0040: (string, LrcPtr<StringBuilder>, i32, i32)) -> Documents::US14 {
                    Documents::closure34((), arg10_0040)
                }
                fn v16(arg10_0040_1: (string, LrcPtr<StringBuilder>, i32, i32)) -> Documents::US14 {
                    Documents::closure37((), arg10_0040_1)
                }
                let v20: Documents::US14 = Documents::method87(
                    v7.clone(),
                    v12.clone(),
                    LrcPtr::new(Documents::UH1::UH1_1(
                        Func1::from(v15),
                        LrcPtr::new(Documents::UH1::UH1_1(
                            Func1::from(v16),
                            LrcPtr::new(Documents::UH1::UH1_0),
                        )),
                    )),
                );
                let v202: Documents::US15 = match &v20 {
                    Documents::US14::US14_0(v20_0_0, v20_0_1, v20_0_2, v20_0_3, v20_0_4) => {
                        let v25: i32 = v20_0_4.clone();
                        let v24: i32 = v20_0_3.clone();
                        let v23: LrcPtr<StringBuilder> = v20_0_2.clone();
                        let v22: string = v20_0_1.clone();
                        let v130: Documents::US14 = if string("") == (v22.clone()) {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                                toArray(ofArray(new_array(&['\"', '\'']))),
                                (v23.clone(), v24, v25)
                            ))
                        } else {
                            let v42: char = getCharAt(v22.clone(), 0_i32);
                            if (Documents::method88(v42, 0_i64)) == false {
                                let v65: string = getSlice(
                                    v22.clone(),
                                    Some(1_i32),
                                    Some((length(v22.clone())) - 1_i32),
                                );
                                let v70: string = ofChar(v42);
                                let v73: i32 = length(v70.clone());
                                let v74: Array<char> = new_init(&'\u{0000}', v73);
                                let v75: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                    l0: MutCell::new(0_i32),
                                });
                                while Documents::method59(v73, v75.clone()) {
                                    let v77: i32 = v75.l0.get().clone();
                                    let v78: char = getCharAt(v70.clone(), v77);
                                    v74.get_mut()[v77 as usize] = v78;
                                    {
                                        let v79: i32 = (v77) + 1_i32;
                                        v75.l0.set(v79);
                                        ()
                                    }
                                }
                                {
                                    let v80: List<char> = ofArray(v74.clone());
                                    let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                        Documents::method86(
                                            foldBack(
                                                Func2::new(
                                                    move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                        (Documents::method85())(b0)(b1)
                                                    },
                                                ),
                                                v80,
                                                LrcPtr::new(Documents::UH0::UH0_0),
                                            ),
                                            v23.clone(),
                                            v24,
                                            v25,
                                        );
                                    Documents::US14::US14_0(
                                        v42,
                                        v65,
                                        patternInput.0.clone(),
                                        patternInput.1.clone(),
                                        patternInput.2.clone(),
                                    )
                                }
                            } else {
                                Documents::US14::US14_1(sprintf!("parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                                                         v42,
                                                                         toArray(ofArray(new_array(&['\"',
                                                                                                     '\'']))),
                                                                         (v23.clone(),
                                                                          v24,
                                                                          v25)))
                            }
                        };
                        let v142: Documents::US14 = match &v130 {
                            Documents::US14::US14_0(
                                v130_0_0,
                                v130_0_1,
                                v130_0_2,
                                v130_0_3,
                                v130_0_4,
                            ) => {
                                let v131: char = v130_0_0.clone();
                                Documents::US14::US14_0(
                                    if '\\' == (v131) { '/' } else { v131 },
                                    v130_0_1.clone(),
                                    v130_0_2.clone(),
                                    v130_0_3.clone(),
                                    v130_0_4.clone(),
                                )
                            }
                            Documents::US14::US14_1(v130_1_0) => {
                                Documents::US14::US14_1(v130_1_0.clone())
                            }
                        };
                        let v161: Documents::US15 = match &v142 {
                            Documents::US14::US14_0(
                                v142_0_0,
                                v142_0_1,
                                v142_0_2,
                                v142_0_3,
                                v142_0_4,
                            ) => {
                                let patternInput_1: (
                                    string,
                                    string,
                                    LrcPtr<StringBuilder>,
                                    i32,
                                    i32,
                                ) = Documents::method89(
                                    ofChar(v142_0_0.clone()),
                                    v142_0_1.clone(),
                                    v142_0_2.clone(),
                                    v142_0_3.clone(),
                                    v142_0_4.clone(),
                                );
                                Documents::US15::US15_0(
                                    patternInput_1.0.clone(),
                                    patternInput_1.1.clone(),
                                    patternInput_1.2.clone(),
                                    patternInput_1.3.clone(),
                                    patternInput_1.4.clone(),
                                )
                            }
                            Documents::US14::US14_1(v142_1_0) => {
                                Documents::US15::US15_1(v142_1_0.clone())
                            }
                        };
                        let v171: Documents::US15 = match &v161 {
                            Documents::US15::US15_0(
                                v161_0_0,
                                v161_0_1,
                                v161_0_2,
                                v161_0_3,
                                v161_0_4,
                            ) => Documents::US15::US15_0(
                                v161_0_0.clone(),
                                v161_0_1.clone(),
                                v161_0_2.clone(),
                                v161_0_3.clone(),
                                v161_0_4.clone(),
                            ),
                            _ => Documents::US15::US15_0(
                                string(""),
                                v22.clone(),
                                v23.clone(),
                                v24,
                                v25,
                            ),
                        };
                        match &v171 {
                            Documents::US15::US15_0(
                                v171_0_0,
                                v171_0_1,
                                v171_0_2,
                                v171_0_3,
                                v171_0_4,
                            ) => {
                                let v176: i32 = v171_0_4.clone();
                                let v175: i32 = v171_0_3.clone();
                                let v174: LrcPtr<StringBuilder> = v171_0_2.clone();
                                let v173: string = v171_0_1.clone();
                                let v180: Documents::US14 = Documents::method90(
                                    v173.clone(),
                                    v174.clone(),
                                    v175,
                                    v176,
                                    LrcPtr::new(Documents::UH1::UH1_1(
                                        Func1::from(v15),
                                        LrcPtr::new(Documents::UH1::UH1_1(
                                            Func1::from(v16),
                                            LrcPtr::new(Documents::UH1::UH1_0),
                                        )),
                                    )),
                                );
                                match &v180 {
                                        Documents::US14::US14_0(v180_0_0,
                                                                v180_0_1,
                                                                v180_0_2,
                                                                v180_0_3,
                                                                v180_0_4) =>
                                        Documents::US15::US15_0(v171_0_0.clone(),
                                                                v180_0_1.clone(),
                                                                v180_0_2.clone(),
                                                                v180_0_3.clone(),
                                                                v180_0_4.clone()),
                                        Documents::US14::US14_1(v180_1_0) =>
                                        Documents::US15::US15_1(sprintf!("parsing.between / expected closing delimiter / e: {:?} / input: {:?} / rest1: {:?} / rest2: {:?}",
                                                                         v180_1_0.clone(),
                                                                         (v7.clone(),
                                                                          v12.clone(),
                                                                          1_i32,
                                                                          1_i32),
                                                                         (v22.clone(),
                                                                          v23.clone(),
                                                                          v24,
                                                                          v25),
                                                                         (v173.clone(),
                                                                          v174.clone(),
                                                                          v175,
                                                                          v176))),
                                    }
                            }
                            _ => Documents::US15::US15_1(string(
                                "parsing.between / expected content",
                            )),
                        }
                    }
                    Documents::US14::US14_1(v20_1_0) => Documents::US15::US15_1(v20_1_0.clone()),
                };
                let v414: Documents::US15 = match &v202 {
                    Documents::US15::US15_0(v202_0_0, v202_0_1, v202_0_2, v202_0_3, v202_0_4) => {
                        v202.clone()
                    }
                    _ => {
                        let v321: Documents::US14 = if string("") == (v7.clone()) {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                                toArray(ofArray(new_array(&['\"', '\'', ' ']))),
                                (v12.clone(), 1_i32, 1_i32)
                            ))
                        } else {
                            let v228: char = getCharAt(v7.clone(), 0_i32);
                            if (Documents::method91(v228, 0_i64)) == false {
                                let v251: string = getSlice(
                                    v7.clone(),
                                    Some(1_i32),
                                    Some((length(v7.clone())) - 1_i32),
                                );
                                let v256: string = ofChar(v228);
                                let v259: i32 = length(v256.clone());
                                let v260: Array<char> = new_init(&'\u{0000}', v259);
                                let v261: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                    l0: MutCell::new(0_i32),
                                });
                                while Documents::method59(v259, v261.clone()) {
                                    let v263: i32 = v261.l0.get().clone();
                                    let v264: char = getCharAt(v256.clone(), v263);
                                    v260.get_mut()[v263 as usize] = v264;
                                    {
                                        let v265: i32 = (v263) + 1_i32;
                                        v261.l0.set(v265);
                                        ()
                                    }
                                }
                                {
                                    let v266: List<char> = ofArray(v260.clone());
                                    let patternInput_2: (LrcPtr<StringBuilder>, i32, i32) =
                                        Documents::method86(
                                            foldBack(
                                                Func2::new(
                                                    move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                        (Documents::method85())(b0)(b1)
                                                    },
                                                ),
                                                v266,
                                                LrcPtr::new(Documents::UH0::UH0_0),
                                            ),
                                            v12.clone(),
                                            1_i32,
                                            1_i32,
                                        );
                                    Documents::US14::US14_0(
                                        v228,
                                        v251,
                                        patternInput_2.0.clone(),
                                        patternInput_2.1.clone(),
                                        patternInput_2.2.clone(),
                                    )
                                }
                            } else {
                                Documents::US14::US14_1(sprintf!("parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                                                         v228,
                                                                         toArray(ofArray(new_array(&['\"',
                                                                                                     '\'',
                                                                                                     ' ']))),
                                                                         (v12.clone(),
                                                                          1_i32,
                                                                          1_i32)))
                            }
                        };
                        let v333: Documents::US14 = match &v321 {
                            Documents::US14::US14_0(
                                v321_0_0,
                                v321_0_1,
                                v321_0_2,
                                v321_0_3,
                                v321_0_4,
                            ) => {
                                let v322: char = v321_0_0.clone();
                                Documents::US14::US14_0(
                                    if '\\' == (v322) { '/' } else { v322 },
                                    v321_0_1.clone(),
                                    v321_0_2.clone(),
                                    v321_0_3.clone(),
                                    v321_0_4.clone(),
                                )
                            }
                            Documents::US14::US14_1(v321_1_0) => {
                                Documents::US14::US14_1(v321_1_0.clone())
                            }
                        };
                        let v352: Documents::US15 = match &v333 {
                            Documents::US14::US14_0(
                                v333_0_0,
                                v333_0_1,
                                v333_0_2,
                                v333_0_3,
                                v333_0_4,
                            ) => {
                                let patternInput_3: (
                                    string,
                                    string,
                                    LrcPtr<StringBuilder>,
                                    i32,
                                    i32,
                                ) = Documents::method92(
                                    ofChar(v333_0_0.clone()),
                                    v333_0_1.clone(),
                                    v333_0_2.clone(),
                                    v333_0_3.clone(),
                                    v333_0_4.clone(),
                                );
                                Documents::US15::US15_0(
                                    patternInput_3.0.clone(),
                                    patternInput_3.1.clone(),
                                    patternInput_3.2.clone(),
                                    patternInput_3.3.clone(),
                                    patternInput_3.4.clone(),
                                )
                            }
                            Documents::US14::US14_1(v333_1_0) => {
                                Documents::US15::US15_1(v333_1_0.clone())
                            }
                        };
                        match &v352 {
                            Documents::US15::US15_0(
                                v352_0_0,
                                v352_0_1,
                                v352_0_2,
                                v352_0_3,
                                v352_0_4,
                            ) => v352.clone(),
                            _ => {
                                let v366: Documents::US17 = if (length(v7.clone())) == 0_i32 {
                                    Documents::US17::US17_0(v7.clone(), v12.clone(), 1_i32, 1_i32)
                                } else {
                                    Documents::US17::US17_1(sprintf!(
                                        "parsing.eof / expected end of input / input: {:?}",
                                        v7.clone()
                                    ))
                                };
                                let v375: Documents::US15 = match &v366 {
                                    Documents::US17::US17_0(
                                        v366_0_0,
                                        v366_0_1,
                                        v366_0_2,
                                        v366_0_3,
                                    ) => Documents::US15::US15_0(
                                        string(""),
                                        v366_0_0.clone(),
                                        v366_0_1.clone(),
                                        v366_0_2.clone(),
                                        v366_0_3.clone(),
                                    ),
                                    Documents::US17::US17_1(v366_1_0) => {
                                        Documents::US15::US15_1(v366_1_0.clone())
                                    }
                                };
                                match &v375 {
                                    Documents::US15::US15_0(
                                        v375_0_0,
                                        v375_0_1,
                                        v375_0_2,
                                        v375_0_3,
                                        v375_0_4,
                                    ) => {
                                        let v377: string = v375_0_1.clone();
                                        Documents::US15::US15_0(
                                            v375_0_0.clone(),
                                            getSlice(
                                                v377.clone(),
                                                Some(Documents::method93(v377.clone(), 0_i32)),
                                                Some((length(v377)) - 1_i32),
                                            ),
                                            v375_0_2.clone(),
                                            v375_0_3.clone(),
                                            v375_0_4.clone(),
                                        )
                                    }
                                    Documents::US15::US15_1(v375_1_0) => {
                                        Documents::US15::US15_1(v375_1_0.clone())
                                    }
                                }
                            }
                        }
                    }
                };
                let v685: Documents::US18 = match &v414 {
                    Documents::US15::US15_0(v414_0_0, v414_0_1, v414_0_2, v414_0_3, v414_0_4) => {
                        let v419: i32 = v414_0_4.clone();
                        let v418: i32 = v414_0_3.clone();
                        let v417: LrcPtr<StringBuilder> = v414_0_2.clone();
                        let v416: string = v414_0_1.clone();
                        let v544: Documents::US14 = if string("") == (v416.clone()) {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                                ' ',
                                (v417.clone(), v418, v419)
                            ))
                        } else {
                            let v425: char = getCharAt(v416.clone(), 0_i32);
                            if (v425) == ' ' {
                                let v446: string = getSlice(
                                    v416.clone(),
                                    Some(1_i32),
                                    Some((length(v416.clone())) - 1_i32),
                                );
                                let v451: string = ofChar(v425);
                                let v454: i32 = length(v451.clone());
                                let v455: Array<char> = new_init(&'\u{0000}', v454);
                                let v456: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                    l0: MutCell::new(0_i32),
                                });
                                while Documents::method59(v454, v456.clone()) {
                                    let v458: i32 = v456.l0.get().clone();
                                    let v459: char = getCharAt(v451.clone(), v458);
                                    v455.get_mut()[v458 as usize] = v459;
                                    {
                                        let v460: i32 = (v458) + 1_i32;
                                        v456.l0.set(v460);
                                        ()
                                    }
                                }
                                {
                                    let v461: List<char> = ofArray(v455.clone());
                                    let patternInput_4: (LrcPtr<StringBuilder>, i32, i32) =
                                        Documents::method86(
                                            foldBack(
                                                Func2::new(
                                                    move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                        (Documents::method85())(b0)(b1)
                                                    },
                                                ),
                                                v461,
                                                LrcPtr::new(Documents::UH0::UH0_0),
                                            ),
                                            v417.clone(),
                                            v418,
                                            v419,
                                        );
                                    Documents::US14::US14_0(
                                        v425,
                                        v446,
                                        patternInput_4.0.clone(),
                                        patternInput_4.1.clone(),
                                        patternInput_4.2.clone(),
                                    )
                                }
                            } else {
                                let v499: i32 = (indexOf(v416.clone(), string("\n"))) - 1_i32;
                                Documents::US14::US14_1(concat(new_array(&[sprintf!("parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                                                                                            ' ',
                                                                                            v418,
                                                                                            v419,
                                                                                            v417.clone(),
                                                                                            getSlice(v416.clone(),
                                                                                                     Some(0_i32),
                                                                                                     Some((if -2_i32
                                                                                                                  ==
                                                                                                                  (v499)
                                                                                                              {
                                                                                                               (length(v416.clone()))
                                                                                                                   +
                                                                                                                   1_i32
                                                                                                           } else {
                                                                                                               (v499)
                                                                                                                   +
                                                                                                                   1_i32
                                                                                                           })
                                                                                                              -
                                                                                                              1_i32))),
                                                                                   string("\n"),
                                                                                   append((replicate((v419)
                                                                                                         -
                                                                                                         1_i32,
                                                                                                     string(" "))),
                                                                                          string("^")),
                                                                                   string("\n")])))
                            }
                        };
                        let v556: Documents::US19 = match &v544 {
                            Documents::US14::US14_0(
                                v544_0_0,
                                v544_0_1,
                                v544_0_2,
                                v544_0_3,
                                v544_0_4,
                            ) => Documents::US19::US19_0(
                                Documents::US16::US16_0(v544_0_0.clone()),
                                v544_0_1.clone(),
                                v544_0_2.clone(),
                                v544_0_3.clone(),
                                v544_0_4.clone(),
                            ),
                            _ => Documents::US19::US19_0(
                                Documents::US16::US16_1,
                                v416.clone(),
                                v417.clone(),
                                v418,
                                v419,
                            ),
                        };
                        let v659: Documents::US15 = match &v556 {
                            Documents::US19::US19_0(
                                v556_0_0,
                                v556_0_1,
                                v556_0_2,
                                v556_0_3,
                                v556_0_4,
                            ) => {
                                let v561: i32 = v556_0_4.clone();
                                let v560: i32 = v556_0_3.clone();
                                let v559: LrcPtr<StringBuilder> = v556_0_2.clone();
                                let v558: string = v556_0_1.clone();
                                let v636: Documents::US14 = if string("") == (v558.clone()) {
                                    Documents::US14::US14_1(sprintf!(
                                        "parsing.any_char / unexpected end of input / s: {:?}",
                                        (v559.clone(), v560, v561)
                                    ))
                                } else {
                                    let v567: char = getCharAt(v558.clone(), 0_i32);
                                    let v587: string = getSlice(
                                        v558.clone(),
                                        Some(1_i32),
                                        Some((length(v558)) - 1_i32),
                                    );
                                    let v592: string = ofChar(v567);
                                    let v595: i32 = length(v592.clone());
                                    let v596: Array<char> = new_init(&'\u{0000}', v595);
                                    let v597: LrcPtr<Documents::Mut6> =
                                        LrcPtr::new(Documents::Mut6 {
                                            l0: MutCell::new(0_i32),
                                        });
                                    while Documents::method59(v595, v597.clone()) {
                                        let v599: i32 = v597.l0.get().clone();
                                        let v600: char = getCharAt(v592.clone(), v599);
                                        v596.get_mut()[v599 as usize] = v600;
                                        {
                                            let v601: i32 = (v599) + 1_i32;
                                            v597.l0.set(v601);
                                            ()
                                        }
                                    }
                                    {
                                        let v602: List<char> = ofArray(v596.clone());
                                        let patternInput_5:
                                                            (LrcPtr<StringBuilder>,
                                                             i32, i32) =
                                                        Documents::method86(foldBack(Func2::new(move
                                                                                                    |b0:
                                                                                                         char,
                                                                                                     b1:
                                                                                                         LrcPtr<Documents::UH0>|
                                                                                                    (Documents::method85())(b0)(b1)),
                                                                                     v602,
                                                                                     LrcPtr::new(Documents::UH0::UH0_0)),
                                                                            v559,
                                                                            v560,
                                                                            v561);
                                        Documents::US14::US14_0(
                                            v567,
                                            v587,
                                            patternInput_5.0.clone(),
                                            patternInput_5.1.clone(),
                                            patternInput_5.2.clone(),
                                        )
                                    }
                                };
                                match &v636 {
                                    Documents::US14::US14_0(
                                        v636_0_0,
                                        v636_0_1,
                                        v636_0_2,
                                        v636_0_3,
                                        v636_0_4,
                                    ) => {
                                        let patternInput_6: (
                                            string,
                                            string,
                                            LrcPtr<StringBuilder>,
                                            i32,
                                            i32,
                                        ) = Documents::method94(
                                            ofChar(v636_0_0.clone()),
                                            v636_0_1.clone(),
                                            v636_0_2.clone(),
                                            v636_0_3.clone(),
                                            v636_0_4.clone(),
                                        );
                                        Documents::US15::US15_0(
                                            patternInput_6.0.clone(),
                                            patternInput_6.1.clone(),
                                            patternInput_6.2.clone(),
                                            patternInput_6.3.clone(),
                                            patternInput_6.4.clone(),
                                        )
                                    }
                                    Documents::US14::US14_1(v636_1_0) => {
                                        Documents::US15::US15_1(v636_1_0.clone())
                                    }
                                }
                            }
                            Documents::US19::US19_1(v556_1_0) => {
                                Documents::US15::US15_1(v556_1_0.clone())
                            }
                        };
                        let v671: Documents::US20 = match &v659 {
                            Documents::US15::US15_0(
                                v659_0_0,
                                v659_0_1,
                                v659_0_2,
                                v659_0_3,
                                v659_0_4,
                            ) => Documents::US20::US20_0(
                                Documents::US5::US5_0(v659_0_0.clone()),
                                v659_0_1.clone(),
                                v659_0_2.clone(),
                                v659_0_3.clone(),
                                v659_0_4.clone(),
                            ),
                            _ => Documents::US20::US20_0(
                                Documents::US5::US5_1,
                                v416.clone(),
                                v417.clone(),
                                v418,
                                v419,
                            ),
                        };
                        match &v671 {
                            Documents::US20::US20_0(
                                v671_0_0,
                                v671_0_1,
                                v671_0_2,
                                v671_0_3,
                                v671_0_4,
                            ) => Documents::US18::US18_0(
                                v414_0_0.clone(),
                                v671_0_0.clone(),
                                v671_0_1.clone(),
                                v671_0_2.clone(),
                                v671_0_3.clone(),
                                v671_0_4.clone(),
                            ),
                            Documents::US20::US20_1(v671_1_0) => {
                                Documents::US18::US18_1(v671_1_0.clone())
                            }
                        }
                    }
                    Documents::US15::US15_1(v414_1_0) => Documents::US18::US18_1(v414_1_0.clone()),
                };
                match &v685 {
                    Documents::US18::US18_0(
                        v685_0_0,
                        v685_0_1,
                        v685_0_2,
                        v685_0_3,
                        v685_0_4,
                        v685_0_5,
                    ) => Documents::US13::US13_0(v685_0_0.clone(), v685_0_1.clone()),
                    Documents::US18::US18_1(v685_1_0) => Documents::US13::US13_1(v685_1_0.clone()),
                }
            }
        }
        pub fn method97(v0_1: char, v1_1: i64) -> bool {
            let v0_1: MutCell<char> = MutCell::new(v0_1);
            let v1_1: MutCell<i64> = MutCell::new(v1_1);
            '_method97: loop {
                break '_method97 (if (v1_1.get().clone()) >= 4_i64 {
                    false
                } else {
                    let v19: Documents::US16 = if (v1_1.get().clone()) == 0_i64 {
                        Documents::US16::US16_0('\\')
                    } else {
                        let v5: i64 = (v1_1.get().clone()) - 1_i64;
                        if (v5) == 0_i64 {
                            Documents::US16::US16_0('`')
                        } else {
                            let v8: i64 = (v5) - 1_i64;
                            if (v8) == 0_i64 {
                                Documents::US16::US16_0('\"')
                            } else {
                                let v11: i64 = (v8) - 1_i64;
                                if (v11) == 0_i64 {
                                    Documents::US16::US16_0(' ')
                                } else {
                                    let v14: i64 = (v11) - 1_i64;
                                    Documents::US16::US16_1
                                }
                            }
                        }
                    };
                    if (v0_1.get().clone())
                        == (match &v19 {
                            Documents::US16::US16_0(v19_0_0) => match &v19 {
                                Documents::US16::US16_0(x) => x.clone(),
                                _ => unreachable!(),
                            },
                            _ => panic!("{}", string("Option does not have a value."),),
                        })
                    {
                        true
                    } else {
                        let v0_1_temp: char = v0_1.get().clone();
                        let v1_1_temp: i64 = (v1_1.get().clone()) + 1_i64;
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method97;
                    }
                });
            }
        }
        pub fn method98(
            v0_1: string,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> (string, string, LrcPtr<StringBuilder>, i32, i32) {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method98: loop {
                break '_method98 ({
                    let v121: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\\', '`', '\"', ' ']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v27: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method97(v27, 0_i64)) == false {
                            let v50: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v55: string = ofChar(v27);
                            let v58: i32 = length(v55.clone());
                            let v59: Array<char> = new_init(&'\u{0000}', v58);
                            let v60: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v58, v60.clone()) {
                                let v62: i32 = v60.l0.get().clone();
                                let v63: char = getCharAt(v55.clone(), v62);
                                v59.get_mut()[v62 as usize] = v63;
                                {
                                    let v64: i32 = (v62) + 1_i32;
                                    v60.l0.set(v64);
                                    ()
                                }
                            }
                            {
                                let v65: List<char> = ofArray(v59.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v65,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v27,
                                    v50,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v27,
                                toArray(ofArray(new_array(&['\\', '`', '\"', ' ']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    match &v121 {
                        Documents::US14::US14_0(
                            v121_0_0,
                            v121_0_1,
                            v121_0_2,
                            v121_0_3,
                            v121_0_4,
                        ) => {
                            let v0_1_temp: string =
                                append((v0_1.get().clone()), (ofChar(v121_0_0.clone())));
                            let v1_1_temp: string = v121_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v121_0_2.clone();
                            let v3_temp: i32 = v121_0_3.clone();
                            let v4_temp: i32 = v121_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method98;
                        }
                        _ => (
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method100(v0_1: char, v1_1: i64) -> bool {
            let v0_1: MutCell<char> = MutCell::new(v0_1);
            let v1_1: MutCell<i64> = MutCell::new(v1_1);
            '_method100: loop {
                break '_method100 (if (v1_1.get().clone()) >= 3_i64 {
                    false
                } else {
                    let v15: Documents::US16 = if (v1_1.get().clone()) == 0_i64 {
                        Documents::US16::US16_0('\\')
                    } else {
                        let v5: i64 = (v1_1.get().clone()) - 1_i64;
                        if (v5) == 0_i64 {
                            Documents::US16::US16_0('`')
                        } else {
                            let v8: i64 = (v5) - 1_i64;
                            if (v8) == 0_i64 {
                                Documents::US16::US16_0('\"')
                            } else {
                                let v11: i64 = (v8) - 1_i64;
                                Documents::US16::US16_1
                            }
                        }
                    };
                    if (v0_1.get().clone())
                        == (match &v15 {
                            Documents::US16::US16_0(v15_0_0) => match &v15 {
                                Documents::US16::US16_0(x) => x.clone(),
                                _ => unreachable!(),
                            },
                            _ => panic!("{}", string("Option does not have a value."),),
                        })
                    {
                        true
                    } else {
                        let v0_1_temp: char = v0_1.get().clone();
                        let v1_1_temp: i64 = (v1_1.get().clone()) + 1_i64;
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method100;
                    }
                });
            }
        }
        pub fn closure38(
            unitVar: (),
            _arg: (string, LrcPtr<StringBuilder>, i32, i32),
        ) -> Documents::US15 {
            let v3: i32 = _arg.3.clone();
            let v2: i32 = _arg.2.clone();
            let v1_1: LrcPtr<StringBuilder> = _arg.1.clone();
            let v0_1: string = _arg.0.clone();
            let v128: Documents::US14 = if string("") == (v0_1.clone()) {
                Documents::US14::US14_1(sprintf!(
                    "parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                    '\\',
                    (v1_1.clone(), v2, v3)
                ))
            } else {
                let v9: char = getCharAt(v0_1.clone(), 0_i32);
                if (v9) == '\\' {
                    let v30: string = getSlice(
                        v0_1.clone(),
                        Some(1_i32),
                        Some((length(v0_1.clone())) - 1_i32),
                    );
                    let v35: string = ofChar(v9);
                    let v38: i32 = length(v35.clone());
                    let v39: Array<char> = new_init(&'\u{0000}', v38);
                    let v40: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                        l0: MutCell::new(0_i32),
                    });
                    while Documents::method59(v38, v40.clone()) {
                        let v42: i32 = v40.l0.get().clone();
                        let v43: char = getCharAt(v35.clone(), v42);
                        v39.get_mut()[v42 as usize] = v43;
                        {
                            let v44: i32 = (v42) + 1_i32;
                            v40.l0.set(v44);
                            ()
                        }
                    }
                    {
                        let v45: List<char> = ofArray(v39.clone());
                        let patternInput: (LrcPtr<StringBuilder>, i32, i32) = Documents::method86(
                            foldBack(
                                Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                    (Documents::method85())(b0)(b1)
                                }),
                                v45,
                                LrcPtr::new(Documents::UH0::UH0_0),
                            ),
                            v1_1.clone(),
                            v2,
                            v3,
                        );
                        Documents::US14::US14_0(
                            v9,
                            v30,
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            patternInput.2.clone(),
                        )
                    }
                } else {
                    let v83: i32 = (indexOf(v0_1.clone(), string("\n"))) - 1_i32;
                    Documents::US14::US14_1(concat(new_array(&[
                        sprintf!(
                            "parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                            '\\',
                            v2,
                            v3,
                            v1_1,
                            getSlice(
                                v0_1.clone(),
                                Some(0_i32),
                                Some(
                                    (if -2_i32 == (v83) {
                                        (length(v0_1)) + 1_i32
                                    } else {
                                        (v83) + 1_i32
                                    }) - 1_i32
                                )
                            )
                        ),
                        string("\n"),
                        append((replicate((v3) - 1_i32, string(" "))), string("^")),
                        string("\n"),
                    ])))
                }
            };
            let v212: Documents::US14 = match &v128 {
                Documents::US14::US14_0(v128_0_0, v128_0_1, v128_0_2, v128_0_3, v128_0_4) => {
                    let v133: i32 = v128_0_4.clone();
                    let v132: i32 = v128_0_3.clone();
                    let v131: LrcPtr<StringBuilder> = v128_0_2.clone();
                    let v130: string = v128_0_1.clone();
                    if string("") == (v130.clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.any_char / unexpected end of input / s: {:?}",
                            (v131.clone(), v132, v133)
                        ))
                    } else {
                        let v139: char = getCharAt(v130.clone(), 0_i32);
                        let v159: string =
                            getSlice(v130.clone(), Some(1_i32), Some((length(v130)) - 1_i32));
                        let v164: string = ofChar(v139);
                        let v167: i32 = length(v164.clone());
                        let v168: Array<char> = new_init(&'\u{0000}', v167);
                        let v169: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                            l0: MutCell::new(0_i32),
                        });
                        while Documents::method59(v167, v169.clone()) {
                            let v171: i32 = v169.l0.get().clone();
                            let v172: char = getCharAt(v164.clone(), v171);
                            v168.get_mut()[v171 as usize] = v172;
                            {
                                let v173: i32 = (v171) + 1_i32;
                                v169.l0.set(v173);
                                ()
                            }
                        }
                        {
                            let v174: List<char> = ofArray(v168.clone());
                            let patternInput_1: (LrcPtr<StringBuilder>, i32, i32) =
                                Documents::method86(
                                    foldBack(
                                        Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                            (Documents::method85())(b0)(b1)
                                        }),
                                        v174,
                                        LrcPtr::new(Documents::UH0::UH0_0),
                                    ),
                                    v131,
                                    v132,
                                    v133,
                                );
                            Documents::US14::US14_0(
                                v139,
                                v159,
                                patternInput_1.0.clone(),
                                patternInput_1.1.clone(),
                                patternInput_1.2.clone(),
                            )
                        }
                    }
                }
                Documents::US14::US14_1(v128_1_0) => Documents::US14::US14_1(v128_1_0.clone()),
            };
            match &v212 {
                Documents::US14::US14_0(v212_0_0, v212_0_1, v212_0_2, v212_0_3, v212_0_4) => {
                    Documents::US15::US15_0(
                        append((ofChar('\\')), (ofChar(v212_0_0.clone()))),
                        v212_0_1.clone(),
                        v212_0_2.clone(),
                        v212_0_3.clone(),
                        v212_0_4.clone(),
                    )
                }
                Documents::US14::US14_1(v212_1_0) => Documents::US15::US15_1(v212_1_0.clone()),
            }
        }
        pub fn closure39(
            unitVar: (),
            _arg: (string, LrcPtr<StringBuilder>, i32, i32),
        ) -> Documents::US15 {
            let v3: i32 = _arg.3.clone();
            let v2: i32 = _arg.2.clone();
            let v1_1: LrcPtr<StringBuilder> = _arg.1.clone();
            let v0_1: string = _arg.0.clone();
            let v128: Documents::US14 = if string("") == (v0_1.clone()) {
                Documents::US14::US14_1(sprintf!(
                    "parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                    '`',
                    (v1_1.clone(), v2, v3)
                ))
            } else {
                let v9: char = getCharAt(v0_1.clone(), 0_i32);
                if (v9) == '`' {
                    let v30: string = getSlice(
                        v0_1.clone(),
                        Some(1_i32),
                        Some((length(v0_1.clone())) - 1_i32),
                    );
                    let v35: string = ofChar(v9);
                    let v38: i32 = length(v35.clone());
                    let v39: Array<char> = new_init(&'\u{0000}', v38);
                    let v40: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                        l0: MutCell::new(0_i32),
                    });
                    while Documents::method59(v38, v40.clone()) {
                        let v42: i32 = v40.l0.get().clone();
                        let v43: char = getCharAt(v35.clone(), v42);
                        v39.get_mut()[v42 as usize] = v43;
                        {
                            let v44: i32 = (v42) + 1_i32;
                            v40.l0.set(v44);
                            ()
                        }
                    }
                    {
                        let v45: List<char> = ofArray(v39.clone());
                        let patternInput: (LrcPtr<StringBuilder>, i32, i32) = Documents::method86(
                            foldBack(
                                Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                    (Documents::method85())(b0)(b1)
                                }),
                                v45,
                                LrcPtr::new(Documents::UH0::UH0_0),
                            ),
                            v1_1.clone(),
                            v2,
                            v3,
                        );
                        Documents::US14::US14_0(
                            v9,
                            v30,
                            patternInput.0.clone(),
                            patternInput.1.clone(),
                            patternInput.2.clone(),
                        )
                    }
                } else {
                    let v83: i32 = (indexOf(v0_1.clone(), string("\n"))) - 1_i32;
                    Documents::US14::US14_1(concat(new_array(&[
                        sprintf!(
                            "parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                            '`',
                            v2,
                            v3,
                            v1_1,
                            getSlice(
                                v0_1.clone(),
                                Some(0_i32),
                                Some(
                                    (if -2_i32 == (v83) {
                                        (length(v0_1)) + 1_i32
                                    } else {
                                        (v83) + 1_i32
                                    }) - 1_i32
                                )
                            )
                        ),
                        string("\n"),
                        append((replicate((v3) - 1_i32, string(" "))), string("^")),
                        string("\n"),
                    ])))
                }
            };
            let v212: Documents::US14 = match &v128 {
                Documents::US14::US14_0(v128_0_0, v128_0_1, v128_0_2, v128_0_3, v128_0_4) => {
                    let v133: i32 = v128_0_4.clone();
                    let v132: i32 = v128_0_3.clone();
                    let v131: LrcPtr<StringBuilder> = v128_0_2.clone();
                    let v130: string = v128_0_1.clone();
                    if string("") == (v130.clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.any_char / unexpected end of input / s: {:?}",
                            (v131.clone(), v132, v133)
                        ))
                    } else {
                        let v139: char = getCharAt(v130.clone(), 0_i32);
                        let v159: string =
                            getSlice(v130.clone(), Some(1_i32), Some((length(v130)) - 1_i32));
                        let v164: string = ofChar(v139);
                        let v167: i32 = length(v164.clone());
                        let v168: Array<char> = new_init(&'\u{0000}', v167);
                        let v169: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                            l0: MutCell::new(0_i32),
                        });
                        while Documents::method59(v167, v169.clone()) {
                            let v171: i32 = v169.l0.get().clone();
                            let v172: char = getCharAt(v164.clone(), v171);
                            v168.get_mut()[v171 as usize] = v172;
                            {
                                let v173: i32 = (v171) + 1_i32;
                                v169.l0.set(v173);
                                ()
                            }
                        }
                        {
                            let v174: List<char> = ofArray(v168.clone());
                            let patternInput_1: (LrcPtr<StringBuilder>, i32, i32) =
                                Documents::method86(
                                    foldBack(
                                        Func2::new(move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                            (Documents::method85())(b0)(b1)
                                        }),
                                        v174,
                                        LrcPtr::new(Documents::UH0::UH0_0),
                                    ),
                                    v131,
                                    v132,
                                    v133,
                                );
                            Documents::US14::US14_0(
                                v139,
                                v159,
                                patternInput_1.0.clone(),
                                patternInput_1.1.clone(),
                                patternInput_1.2.clone(),
                            )
                        }
                    }
                }
                Documents::US14::US14_1(v128_1_0) => Documents::US14::US14_1(v128_1_0.clone()),
            };
            match &v212 {
                Documents::US14::US14_0(v212_0_0, v212_0_1, v212_0_2, v212_0_3, v212_0_4) => {
                    Documents::US15::US15_0(
                        append((ofChar('`')), (ofChar(v212_0_0.clone()))),
                        v212_0_1.clone(),
                        v212_0_2.clone(),
                        v212_0_3.clone(),
                        v212_0_4.clone(),
                    )
                }
                Documents::US14::US14_1(v212_1_0) => Documents::US15::US15_1(v212_1_0.clone()),
            }
        }
        pub fn method101(
            v0_1: string,
            v1_1: LrcPtr<StringBuilder>,
            v2: i32,
            v3: i32,
            v4: LrcPtr<Documents::UH3>,
        ) -> Documents::US15 {
            let v0_1: MutCell<string> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v1_1.clone());
            let v2: MutCell<i32> = MutCell::new(v2);
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<LrcPtr<Documents::UH3>> = MutCell::new(v4.clone());
            '_method101: loop {
                break '_method101 (match v4.get().clone().as_ref() {
                    Documents::UH3::UH3_0 => {
                        Documents::US15::US15_1(string("parsing.choice / no parsers succeeded"))
                    }
                    Documents::UH3::UH3_1(v4_1_0, v4_1_1) => {
                        let v9: Documents::US15 = (match v4.get().clone().as_ref() {
                            Documents::UH3::UH3_1(x, _) => x.clone(),
                            _ => unreachable!(),
                        })((
                            v0_1.get().clone(),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                        ));
                        match &v9 {
                            Documents::US15::US15_0(v9_0_0, v9_0_1, v9_0_2, v9_0_3, v9_0_4) => {
                                v9.clone()
                            }
                            _ => {
                                let v0_1_temp: string = v0_1.get().clone();
                                let v1_1_temp: LrcPtr<StringBuilder> = v1_1.get().clone();
                                let v2_temp: i32 = v2.get().clone();
                                let v3_temp: i32 = v3.get().clone();
                                let v4_temp: LrcPtr<Documents::UH3> =
                                    match v4.get().clone().as_ref() {
                                        Documents::UH3::UH3_1(_, x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone();
                                v0_1.set(v0_1_temp);
                                v1_1.set(v1_1_temp);
                                v2.set(v2_temp);
                                v3.set(v3_temp);
                                v4.set(v4_temp);
                                continue '_method101;
                            }
                        }
                    }
                });
            }
        }
        pub fn method102(
            v0_1: LrcPtr<Documents::UH2>,
            v1_1: LrcPtr<Documents::UH2>,
        ) -> LrcPtr<Documents::UH2> {
            let v0_1: MutCell<LrcPtr<Documents::UH2>> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<LrcPtr<Documents::UH2>> = MutCell::new(v1_1.clone());
            '_method102: loop {
                break '_method102 (match v0_1.get().clone().as_ref() {
                    Documents::UH2::UH2_0 => v1_1.get().clone(),
                    Documents::UH2::UH2_1(v0_1_1_0, v0_1_1_1) => {
                        let v0_1_temp: LrcPtr<Documents::UH2> = match v0_1.get().clone().as_ref() {
                            Documents::UH2::UH2_1(_, x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone();
                        let v1_1_temp: LrcPtr<Documents::UH2> = LrcPtr::new(Documents::UH2::UH2_1(
                            match v0_1.get().clone().as_ref() {
                                Documents::UH2::UH2_1(x, _) => x.clone(),
                                _ => unreachable!(),
                            }
                            .clone(),
                            v1_1.get().clone(),
                        ));
                        v0_1.set(v0_1_temp);
                        v1_1.set(v1_1_temp);
                        continue '_method102;
                    }
                });
            }
        }
        pub fn method99(
            v0_1: LrcPtr<Documents::UH2>,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> Documents::US22 {
            let v0_1: MutCell<LrcPtr<Documents::UH2>> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method99: loop {
                break '_method99 ({
                    let v115: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\\', '`', '\"']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v24: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method100(v24, 0_i64)) == false {
                            let v47: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v52: string = ofChar(v24);
                            let v55: i32 = length(v52.clone());
                            let v56: Array<char> = new_init(&'\u{0000}', v55);
                            let v57: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v55, v57.clone()) {
                                let v59: i32 = v57.l0.get().clone();
                                let v60: char = getCharAt(v52.clone(), v59);
                                v56.get_mut()[v59 as usize] = v60;
                                {
                                    let v61: i32 = (v59) + 1_i32;
                                    v57.l0.set(v61);
                                    ()
                                }
                            }
                            {
                                let v62: List<char> = ofArray(v56.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v62,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v24,
                                    v47,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v24,
                                toArray(ofArray(new_array(&['\\', '`', '\"']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    let v129: Documents::US15 = match &v115 {
                        Documents::US14::US14_0(
                            v115_0_0,
                            v115_0_1,
                            v115_0_2,
                            v115_0_3,
                            v115_0_4,
                        ) => Documents::US15::US15_0(
                            ofChar(v115_0_0.clone()),
                            v115_0_1.clone(),
                            v115_0_2.clone(),
                            v115_0_3.clone(),
                            v115_0_4.clone(),
                        ),
                        Documents::US14::US14_1(v115_1_0) => {
                            Documents::US15::US15_1(v115_1_0.clone())
                        }
                    };
                    let v143: Documents::US15 = match &v129 {
                        Documents::US15::US15_0(
                            v129_0_0,
                            v129_0_1,
                            v129_0_2,
                            v129_0_3,
                            v129_0_4,
                        ) => v129.clone(),
                        _ => Documents::method101(
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                            LrcPtr::new(Documents::UH3::UH3_1(
                                Func1::new(
                                    move |arg10_0040: (string, LrcPtr<StringBuilder>, i32, i32)| {
                                        Documents::closure38((), arg10_0040)
                                    },
                                ),
                                LrcPtr::new(Documents::UH3::UH3_1(
                                    Func1::new(
                                        move |arg10_0040_1: (
                                            string,
                                            LrcPtr<StringBuilder>,
                                            i32,
                                            i32,
                                        )| {
                                            Documents::closure39((), arg10_0040_1)
                                        },
                                    ),
                                    LrcPtr::new(Documents::UH3::UH3_0),
                                )),
                            )),
                        ),
                    };
                    match &v143 {
                        Documents::US15::US15_0(
                            v143_0_0,
                            v143_0_1,
                            v143_0_2,
                            v143_0_3,
                            v143_0_4,
                        ) => {
                            let v0_1_temp: LrcPtr<Documents::UH2> = LrcPtr::new(
                                Documents::UH2::UH2_1(v143_0_0.clone(), v0_1.get().clone()),
                            );
                            let v1_1_temp: string = v143_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v143_0_2.clone();
                            let v3_temp: i32 = v143_0_3.clone();
                            let v4_temp: i32 = v143_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method99;
                        }
                        _ => Documents::US22::US22_0(
                            Documents::method102(
                                v0_1.get().clone(),
                                LrcPtr::new(Documents::UH2::UH2_0),
                            ),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method103(v0_1: LrcPtr<Documents::UH2>, v1_1: List<string>) -> List<string> {
            match v0_1.as_ref() {
                Documents::UH2::UH2_0 => v1_1.clone(),
                Documents::UH2::UH2_1(v0_1_1_0, v0_1_1_1) => cons(
                    match v0_1.as_ref() {
                        Documents::UH2::UH2_1(x, _) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    Documents::method103(
                        match v0_1.as_ref() {
                            Documents::UH2::UH2_1(_, x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        v1_1.clone(),
                    ),
                ),
            }
        }
        pub fn method104(
            v0_1: LrcPtr<Documents::UH2>,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> Documents::US22 {
            let v0_1: MutCell<LrcPtr<Documents::UH2>> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method104: loop {
                break '_method104 ({
                    let v115: Documents::US14 = if string("") == (v1_1.get().clone()) {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\\', '`', '\"']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v24: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method100(v24, 0_i64)) == false {
                            let v47: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v52: string = ofChar(v24);
                            let v55: i32 = length(v52.clone());
                            let v56: Array<char> = new_init(&'\u{0000}', v55);
                            let v57: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v55, v57.clone()) {
                                let v59: i32 = v57.l0.get().clone();
                                let v60: char = getCharAt(v52.clone(), v59);
                                v56.get_mut()[v59 as usize] = v60;
                                {
                                    let v61: i32 = (v59) + 1_i32;
                                    v57.l0.set(v61);
                                    ()
                                }
                            }
                            {
                                let v62: List<char> = ofArray(v56.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v62,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v24,
                                    v47,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v24,
                                toArray(ofArray(new_array(&['\\', '`', '\"']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    let v129: Documents::US15 = match &v115 {
                        Documents::US14::US14_0(
                            v115_0_0,
                            v115_0_1,
                            v115_0_2,
                            v115_0_3,
                            v115_0_4,
                        ) => Documents::US15::US15_0(
                            ofChar(v115_0_0.clone()),
                            v115_0_1.clone(),
                            v115_0_2.clone(),
                            v115_0_3.clone(),
                            v115_0_4.clone(),
                        ),
                        Documents::US14::US14_1(v115_1_0) => {
                            Documents::US15::US15_1(v115_1_0.clone())
                        }
                    };
                    match &v129 {
                        Documents::US15::US15_0(
                            v129_0_0,
                            v129_0_1,
                            v129_0_2,
                            v129_0_3,
                            v129_0_4,
                        ) => {
                            let v0_1_temp: LrcPtr<Documents::UH2> = LrcPtr::new(
                                Documents::UH2::UH2_1(v129_0_0.clone(), v0_1.get().clone()),
                            );
                            let v1_1_temp: string = v129_0_1.clone();
                            let v2_temp: LrcPtr<StringBuilder> = v129_0_2.clone();
                            let v3_temp: i32 = v129_0_3.clone();
                            let v4_temp: i32 = v129_0_4.clone();
                            v0_1.set(v0_1_temp);
                            v1_1.set(v1_1_temp);
                            v2.set(v2_temp);
                            v3.set(v3_temp);
                            v4.set(v4_temp);
                            continue '_method104;
                        }
                        _ => Documents::US22::US22_0(
                            Documents::method102(
                                v0_1.get().clone(),
                                LrcPtr::new(Documents::UH2::UH2_0),
                            ),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method96(
            v0_1: LrcPtr<Documents::UH2>,
            v1_1: string,
            v2: LrcPtr<StringBuilder>,
            v3: i32,
            v4: i32,
        ) -> Documents::US22 {
            let v0_1: MutCell<LrcPtr<Documents::UH2>> = MutCell::new(v0_1.clone());
            let v1_1: MutCell<string> = MutCell::new(v1_1.clone());
            let v2: MutCell<LrcPtr<StringBuilder>> = MutCell::new(v2.clone());
            let v3: MutCell<i32> = MutCell::new(v3);
            let v4: MutCell<i32> = MutCell::new(v4);
            '_method96: loop {
                break '_method96 ({
                    let v5: bool = string("") == (v1_1.get().clone());
                    let v121: Documents::US14 = if v5 {
                        Documents::US14::US14_1(sprintf!(
                            "parsing.none_of / unexpected end of input / chars: {:?} / s: {:?}",
                            toArray(ofArray(new_array(&['\\', '`', '\"', ' ']))),
                            (v2.get().clone(), v3.get().clone(), v4.get().clone())
                        ))
                    } else {
                        let v27: char = getCharAt(v1_1.get().clone(), 0_i32);
                        if (Documents::method97(v27, 0_i64)) == false {
                            let v50: string = getSlice(
                                v1_1.get().clone(),
                                Some(1_i32),
                                Some((length(v1_1.get().clone())) - 1_i32),
                            );
                            let v55: string = ofChar(v27);
                            let v58: i32 = length(v55.clone());
                            let v59: Array<char> = new_init(&'\u{0000}', v58);
                            let v60: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                                l0: MutCell::new(0_i32),
                            });
                            while Documents::method59(v58, v60.clone()) {
                                let v62: i32 = v60.l0.get().clone();
                                let v63: char = getCharAt(v55.clone(), v62);
                                v59.get_mut()[v62 as usize] = v63;
                                {
                                    let v64: i32 = (v62) + 1_i32;
                                    v60.l0.set(v64);
                                    ()
                                }
                            }
                            {
                                let v65: List<char> = ofArray(v59.clone());
                                let patternInput: (LrcPtr<StringBuilder>, i32, i32) =
                                    Documents::method86(
                                        foldBack(
                                            Func2::new(
                                                move |b0: char, b1: LrcPtr<Documents::UH0>| {
                                                    (Documents::method85())(b0)(b1)
                                                },
                                            ),
                                            v65,
                                            LrcPtr::new(Documents::UH0::UH0_0),
                                        ),
                                        v2.get().clone(),
                                        v3.get().clone(),
                                        v4.get().clone(),
                                    );
                                Documents::US14::US14_0(
                                    v27,
                                    v50,
                                    patternInput.0.clone(),
                                    patternInput.1.clone(),
                                    patternInput.2.clone(),
                                )
                            }
                        } else {
                            Documents::US14::US14_1(sprintf!(
                                "parsing.none_of / unexpected char: \'{}\' / chars: {:?} / s: {:?}",
                                v27,
                                toArray(ofArray(new_array(&['\\', '`', '\"', ' ']))),
                                (v2.get().clone(), v3.get().clone(), v4.get().clone())
                            ))
                        }
                    };
                    let v140: Documents::US15 = match &v121 {
                        Documents::US14::US14_0(
                            v121_0_0,
                            v121_0_1,
                            v121_0_2,
                            v121_0_3,
                            v121_0_4,
                        ) => {
                            let patternInput_1: (string, string, LrcPtr<StringBuilder>, i32, i32) =
                                Documents::method98(
                                    ofChar(v121_0_0.clone()),
                                    v121_0_1.clone(),
                                    v121_0_2.clone(),
                                    v121_0_3.clone(),
                                    v121_0_4.clone(),
                                );
                            Documents::US15::US15_0(
                                patternInput_1.0.clone(),
                                patternInput_1.1.clone(),
                                patternInput_1.2.clone(),
                                patternInput_1.3.clone(),
                                patternInput_1.4.clone(),
                            )
                        }
                        Documents::US14::US14_1(v121_1_0) => {
                            Documents::US15::US15_1(v121_1_0.clone())
                        }
                    };
                    let v521: Documents::US15 = match &v140 {
                        Documents::US15::US15_0(
                            v140_0_0,
                            v140_0_1,
                            v140_0_2,
                            v140_0_3,
                            v140_0_4,
                        ) => v140.clone(),
                        _ => {
                            let v270: Documents::US14 = if v5 {
                                Documents::US14::US14_1(sprintf!("parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                                                                                  '\"',
                                                                                  (v2.get().clone(),
                                                                                   v3.get().clone(),
                                                                                   v4.get().clone())))
                            } else {
                                let v151: char = getCharAt(v1_1.get().clone(), 0_i32);
                                if (v151) == '\"' {
                                    let v172: string = getSlice(
                                        v1_1.get().clone(),
                                        Some(1_i32),
                                        Some((length(v1_1.get().clone())) - 1_i32),
                                    );
                                    let v177: string = ofChar(v151);
                                    let v180: i32 = length(v177.clone());
                                    let v181: Array<char> = new_init(&'\u{0000}', v180);
                                    let v182: LrcPtr<Documents::Mut6> =
                                        LrcPtr::new(Documents::Mut6 {
                                            l0: MutCell::new(0_i32),
                                        });
                                    while Documents::method59(v180, v182.clone()) {
                                        let v184: i32 = v182.l0.get().clone();
                                        let v185: char = getCharAt(v177.clone(), v184);
                                        v181.get_mut()[v184 as usize] = v185;
                                        {
                                            let v186: i32 = (v184) + 1_i32;
                                            v182.l0.set(v186);
                                            ()
                                        }
                                    }
                                    {
                                        let v187: List<char> = ofArray(v181.clone());
                                        let patternInput_2:
                                                                 (LrcPtr<StringBuilder>,
                                                                  i32, i32) =
                                                             Documents::method86(foldBack(Func2::new(move
                                                                                                         |b0:
                                                                                                              char,
                                                                                                          b1:
                                                                                                              LrcPtr<Documents::UH0>|
                                                                                                         (Documents::method85())(b0)(b1)),
                                                                                          v187,
                                                                                          LrcPtr::new(Documents::UH0::UH0_0)),
                                                                                 v2.get().clone(),
                                                                                 v3.get().clone(),
                                                                                 v4.get().clone());
                                        Documents::US14::US14_0(
                                            v151,
                                            v172,
                                            patternInput_2.0.clone(),
                                            patternInput_2.1.clone(),
                                            patternInput_2.2.clone(),
                                        )
                                    }
                                } else {
                                    let v225: i32 =
                                        (indexOf(v1_1.get().clone(), string("\n"))) - 1_i32;
                                    Documents::US14::US14_1(concat(new_array(&[sprintf!("parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                                                                                                         '\"',
                                                                                                         v3.get().clone(),
                                                                                                         v4.get().clone(),
                                                                                                         v2.get().clone(),
                                                                                                         getSlice(v1_1.get().clone(),
                                                                                                                  Some(0_i32),
                                                                                                                  Some((if -2_i32
                                                                                                                               ==
                                                                                                                               (v225)
                                                                                                                           {
                                                                                                                            (length(v1_1.get().clone()))
                                                                                                                                +
                                                                                                                                1_i32
                                                                                                                        } else {
                                                                                                                            (v225)
                                                                                                                                +
                                                                                                                                1_i32
                                                                                                                        })
                                                                                                                           -
                                                                                                                           1_i32))),
                                                                                                string("\n"),
                                                                                                append((replicate((v4.get().clone())
                                                                                                                      -
                                                                                                                      1_i32,
                                                                                                                  string(" "))),
                                                                                                       string("^")),
                                                                                                string("\n")])))
                                }
                            };
                            let v456: Documents::US15 = match &v270 {
                                Documents::US14::US14_0(
                                    v270_0_0,
                                    v270_0_1,
                                    v270_0_2,
                                    v270_0_3,
                                    v270_0_4,
                                ) => {
                                    let v275: i32 = v270_0_4.clone();
                                    let v274: i32 = v270_0_3.clone();
                                    let v273: LrcPtr<StringBuilder> = v270_0_2.clone();
                                    let v272: string = v270_0_1.clone();
                                    let v277: Documents::US22 = Documents::method99(
                                        LrcPtr::new(Documents::UH2::UH2_0),
                                        v272.clone(),
                                        v273.clone(),
                                        v274,
                                        v275,
                                    );
                                    let v304: Documents::US15 = match &v277 {
                                        Documents::US22::US22_0(
                                            v277_0_0,
                                            v277_0_1,
                                            v277_0_2,
                                            v277_0_3,
                                            v277_0_4,
                                        ) => {
                                            let v284: List<string> = Documents::method103(
                                                v277_0_0.clone(),
                                                empty::<string>(),
                                            );
                                            Documents::US15::US15_0(
                                                join(
                                                    string(""),
                                                    toArray_1(delay(Func0::new({
                                                        let v284 = v284.clone();
                                                        move || {
                                                            map_1(
                                                                Func1::new({
                                                                    let v284 = v284.clone();
                                                                    move |i: i32| {
                                                                        item(i, v284.clone())
                                                                    }
                                                                }),
                                                                rangeNumeric(
                                                                    0_i32,
                                                                    1_i32,
                                                                    (length_1(v284.clone()))
                                                                        - 1_i32,
                                                                ),
                                                            )
                                                        }
                                                    }))),
                                                ),
                                                v277_0_1.clone(),
                                                v277_0_2.clone(),
                                                v277_0_3.clone(),
                                                v277_0_4.clone(),
                                            )
                                        }
                                        Documents::US22::US22_1(v277_1_0) => {
                                            Documents::US15::US15_1(v277_1_0.clone())
                                        }
                                    };
                                    match &v304 {
                                        Documents::US15::US15_0(
                                            v304_0_0,
                                            v304_0_1,
                                            v304_0_2,
                                            v304_0_3,
                                            v304_0_4,
                                        ) => {
                                            let v309: i32 = v304_0_4.clone();
                                            let v308: i32 = v304_0_3.clone();
                                            let v307: LrcPtr<StringBuilder> = v304_0_2.clone();
                                            let v306: string = v304_0_1.clone();
                                            let v434: Documents::US14 = if string("")
                                                == (v306.clone())
                                            {
                                                Documents::US14::US14_1(sprintf!("parsing.p_char / unexpected end of input / c: \'{}\' / s: {:?}",
                                                                                                      '\"',
                                                                                                      (v307.clone(),
                                                                                                       v308,
                                                                                                       v309)))
                                            } else {
                                                let v315: char = getCharAt(v306.clone(), 0_i32);
                                                if (v315) == '\"' {
                                                    let v336: string = getSlice(
                                                        v306.clone(),
                                                        Some(1_i32),
                                                        Some((length(v306.clone())) - 1_i32),
                                                    );
                                                    let v341: string = ofChar(v315);
                                                    let v344: i32 = length(v341.clone());
                                                    let v345: Array<char> =
                                                        new_init(&'\u{0000}', v344);
                                                    let v346: LrcPtr<Documents::Mut6> =
                                                        LrcPtr::new(Documents::Mut6 {
                                                            l0: MutCell::new(0_i32),
                                                        });
                                                    while Documents::method59(v344, v346.clone()) {
                                                        let v348: i32 = v346.l0.get().clone();
                                                        let v349: char =
                                                            getCharAt(v341.clone(), v348);
                                                        v345.get_mut()[v348 as usize] = v349;
                                                        {
                                                            let v350: i32 = (v348) + 1_i32;
                                                            v346.l0.set(v350);
                                                            ()
                                                        }
                                                    }
                                                    {
                                                        let v351: List<char> =
                                                            ofArray(v345.clone());
                                                        let patternInput_3:
                                                                                     (LrcPtr<StringBuilder>,
                                                                                      i32,
                                                                                      i32) =
                                                                                 Documents::method86(foldBack(Func2::new(move
                                                                                                                             |b0:
                                                                                                                                  char,
                                                                                                                              b1:
                                                                                                                                  LrcPtr<Documents::UH0>|
                                                                                                                             (Documents::method85())(b0)(b1)),
                                                                                                              v351,
                                                                                                              LrcPtr::new(Documents::UH0::UH0_0)),
                                                                                                     v307.clone(),
                                                                                                     v308,
                                                                                                     v309);
                                                        Documents::US14::US14_0(
                                                            v315,
                                                            v336,
                                                            patternInput_3.0.clone(),
                                                            patternInput_3.1.clone(),
                                                            patternInput_3.2.clone(),
                                                        )
                                                    }
                                                } else {
                                                    let v389: i32 =
                                                        (indexOf(v306.clone(), string("\n")))
                                                            - 1_i32;
                                                    Documents::US14::US14_1(concat(new_array(&[sprintf!("parsing.p_char / expected: \'{}\' / line: {} / col: {}\n{}{}",
                                                                                                                             '\"',
                                                                                                                             v308,
                                                                                                                             v309,
                                                                                                                             v307.clone(),
                                                                                                                             getSlice(v306.clone(),
                                                                                                                                      Some(0_i32),
                                                                                                                                      Some((if -2_i32
                                                                                                                                                   ==
                                                                                                                                                   (v389)
                                                                                                                                               {
                                                                                                                                                (length(v306.clone()))
                                                                                                                                                    +
                                                                                                                                                    1_i32
                                                                                                                                            } else {
                                                                                                                                                (v389)
                                                                                                                                                    +
                                                                                                                                                    1_i32
                                                                                                                                            })
                                                                                                                                               -
                                                                                                                                               1_i32))),
                                                                                                                    string("\n"),
                                                                                                                    append((replicate((v309)
                                                                                                                                          -
                                                                                                                                          1_i32,
                                                                                                                                      string(" "))),
                                                                                                                           string("^")),
                                                                                                                    string("\n")])))
                                                }
                                            };
                                            match &v434 {
                                                                 Documents::US14::US14_0(v434_0_0,
                                                                                         v434_0_1,
                                                                                         v434_0_2,
                                                                                         v434_0_3,
                                                                                         v434_0_4)
                                                                 =>
                                                                 Documents::US15::US15_0(v304_0_0.clone(),
                                                                                         v434_0_1.clone(),
                                                                                         v434_0_2.clone(),
                                                                                         v434_0_3.clone(),
                                                                                         v434_0_4.clone()),
                                                                 Documents::US14::US14_1(v434_1_0)
                                                                 =>
                                                                 Documents::US15::US15_1(sprintf!("parsing.between / expected closing delimiter / e: {:?} / input: {:?} / rest1: {:?} / rest2: {:?}",
                                                                                                  v434_1_0.clone(),
                                                                                                  (v1_1.get().clone(),
                                                                                                   v2.get().clone(),
                                                                                                   v3.get().clone(),
                                                                                                   v4.get().clone()),
                                                                                                  (v272.clone(),
                                                                                                   v273.clone(),
                                                                                                   v274,
                                                                                                   v275),
                                                                                                  (v306.clone(),
                                                                                                   v307.clone(),
                                                                                                   v308,
                                                                                                   v309))),
                                                             }
                                        }
                                        _ => Documents::US15::US15_1(string(
                                            "parsing.between / expected content",
                                        )),
                                    }
                                }
                                Documents::US14::US14_1(v270_1_0) => {
                                    Documents::US15::US15_1(v270_1_0.clone())
                                }
                            };
                            match &v456 {
                                Documents::US15::US15_0(
                                    v456_0_0,
                                    v456_0_1,
                                    v456_0_2,
                                    v456_0_3,
                                    v456_0_4,
                                ) => v456.clone(),
                                _ => {
                                    let v468: Documents::US15 =
                                                     Documents::method101(v1_1.get().clone(),
                                                                          v2.get().clone(),
                                                                          v3.get().clone(),
                                                                          v4.get().clone(),
                                                                          LrcPtr::new(Documents::UH3::UH3_1(Func1::new(move
                                                                                                                           |arg10_0040:
                                                                                                                                (string,
                                                                                                                                 LrcPtr<StringBuilder>,
                                                                                                                                 i32,
                                                                                                                                 i32)|
                                                                                                                           Documents::closure38((),
                                                                                                                                                arg10_0040)),
                                                                                                            LrcPtr::new(Documents::UH3::UH3_1(Func1::new(move
                                                                                                                                                             |arg10_0040_1:
                                                                                                                                                                  (string,
                                                                                                                                                                   LrcPtr<StringBuilder>,
                                                                                                                                                                   i32,
                                                                                                                                                                   i32)|
                                                                                                                                                             Documents::closure39((),
                                                                                                                                                                                  arg10_0040_1)),
                                                                                                                                              LrcPtr::new(Documents::UH3::UH3_0))))));
                                    let v479: Documents::US15 = match &v468 {
                                        Documents::US15::US15_0(
                                            v468_0_0,
                                            v468_0_1,
                                            v468_0_2,
                                            v468_0_3,
                                            v468_0_4,
                                        ) => Documents::US15::US15_0(
                                            string(""),
                                            v468_0_1.clone(),
                                            v468_0_2.clone(),
                                            v468_0_3.clone(),
                                            v468_0_4.clone(),
                                        ),
                                        Documents::US15::US15_1(v468_1_0) => {
                                            Documents::US15::US15_1(v468_1_0.clone())
                                        }
                                    };
                                    let v490: Documents::US22 = match &v479 {
                                        Documents::US15::US15_0(
                                            v479_0_0,
                                            v479_0_1,
                                            v479_0_2,
                                            v479_0_3,
                                            v479_0_4,
                                        ) => Documents::method104(
                                            LrcPtr::new(Documents::UH2::UH2_0),
                                            v479_0_1.clone(),
                                            v479_0_2.clone(),
                                            v479_0_3.clone(),
                                            v479_0_4.clone(),
                                        ),
                                        Documents::US15::US15_1(v479_1_0) => {
                                            Documents::US22::US22_1(v479_1_0.clone())
                                        }
                                    };
                                    match &v490 {
                                        Documents::US22::US22_0(
                                            v490_0_0,
                                            v490_0_1,
                                            v490_0_2,
                                            v490_0_3,
                                            v490_0_4,
                                        ) => {
                                            let v497: List<string> = Documents::method103(
                                                v490_0_0.clone(),
                                                empty::<string>(),
                                            );
                                            Documents::US15::US15_0(
                                                join(
                                                    string(""),
                                                    toArray_1(delay(Func0::new({
                                                        let v497 = v497.clone();
                                                        move || {
                                                            map_1(
                                                                Func1::new({
                                                                    let v497 = v497.clone();
                                                                    move |i_1: i32| {
                                                                        item(i_1, v497.clone())
                                                                    }
                                                                }),
                                                                rangeNumeric(
                                                                    0_i32,
                                                                    1_i32,
                                                                    (length_1(v497.clone()))
                                                                        - 1_i32,
                                                                ),
                                                            )
                                                        }
                                                    }))),
                                                ),
                                                v490_0_1.clone(),
                                                v490_0_2.clone(),
                                                v490_0_3.clone(),
                                                v490_0_4.clone(),
                                            )
                                        }
                                        Documents::US22::US22_1(v490_1_0) => {
                                            Documents::US15::US15_1(v490_1_0.clone())
                                        }
                                    }
                                }
                            }
                        }
                    };
                    match &v521 {
                        Documents::US15::US15_0(
                            v521_0_0,
                            v521_0_1,
                            v521_0_2,
                            v521_0_3,
                            v521_0_4,
                        ) => {
                            let v526: i32 = v521_0_4.clone();
                            let v525: i32 = v521_0_3.clone();
                            let v524: LrcPtr<StringBuilder> = v521_0_2.clone();
                            let v523: string = v521_0_1.clone();
                            let v522: string = v521_0_0.clone();
                            let v528: i32 = Documents::method93(v523.clone(), 0_i32);
                            let v556: Documents::US17 = if 0_i32 == (v528) {
                                Documents::US17::US17_1(string(
                                    "parsing.spaces1 / expected at least one space",
                                ))
                            } else {
                                Documents::US17::US17_0(
                                    getSlice(
                                        v523.clone(),
                                        Some(v528),
                                        Some((length(v523.clone())) - 1_i32),
                                    ),
                                    v524.clone(),
                                    v525,
                                    v526,
                                )
                            };
                            match &v556 {
                                Documents::US17::US17_0(v556_0_0, v556_0_1, v556_0_2, v556_0_3) => {
                                    let v0_1_temp: LrcPtr<Documents::UH2> = LrcPtr::new(
                                        Documents::UH2::UH2_1(v522.clone(), v0_1.get().clone()),
                                    );
                                    let v1_1_temp: string = v556_0_0.clone();
                                    let v2_temp: LrcPtr<StringBuilder> = v556_0_1.clone();
                                    let v3_temp: i32 = v556_0_2.clone();
                                    let v4_temp: i32 = v556_0_3.clone();
                                    v0_1.set(v0_1_temp);
                                    v1_1.set(v1_1_temp);
                                    v2.set(v2_temp);
                                    v3.set(v3_temp);
                                    v4.set(v4_temp);
                                    continue '_method96;
                                }
                                _ => Documents::US22::US22_0(
                                    Documents::method102(
                                        v0_1.get().clone(),
                                        LrcPtr::new(Documents::UH2::UH2_1(
                                            v522.clone(),
                                            LrcPtr::new(Documents::UH2::UH2_0),
                                        )),
                                    ),
                                    v523.clone(),
                                    v524.clone(),
                                    v525,
                                    v526,
                                ),
                            }
                        }
                        _ => Documents::US22::US22_0(
                            Documents::method102(
                                v0_1.get().clone(),
                                LrcPtr::new(Documents::UH2::UH2_0),
                            ),
                            v1_1.get().clone(),
                            v2.get().clone(),
                            v3.get().clone(),
                            v4.get().clone(),
                        ),
                    }
                });
            }
        }
        pub fn method95(v0_1: string) -> Documents::US21 {
            let _v0: MutCell<Option<Option<string>>> = MutCell::new(None::<Option<string>>);
            _v0.set(Some(Some(v0_1)));
            {
                let v18: Documents::US22 = Documents::method96(
                    LrcPtr::new(Documents::UH2::UH2_0),
                    defaultValue(
                        string(""),
                        match &_v0.get().clone() {
                            None => panic!("{}", string("optionm\'.of_obj / _v0=None"),),
                            Some(_v0_0_0) => _v0_0_0.clone(),
                        },
                    ),
                    StringBuilder::_ctor__Z721C83C5(Documents::method84()),
                    1_i32,
                    1_i32,
                );
                match &v18 {
                    Documents::US22::US22_0(v18_0_0, v18_0_1, v18_0_2, v18_0_3, v18_0_4) => {
                        Documents::US21::US21_0(toArray(Documents::method103(
                            v18_0_0.clone(),
                            empty::<string>(),
                        )))
                    }
                    Documents::US22::US22_1(v18_1_0) => Documents::US21::US21_1(v18_1_0.clone()),
                }
            }
        }
        pub fn method106(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: Option<CancellationToken>,
            v4: Array<(string, string)>,
            v5: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v6: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v7: bool,
            v8: Option<string>,
        ) -> string {
            let v10: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v17: () = {
                Documents::closure8(v10.clone(), string("{ "), ());
                ()
            };
            let v26: () = {
                Documents::closure8(v10.clone(), string("file_name"), ());
                ()
            };
            let v35: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v43: () = {
                Documents::closure8(v10.clone(), v0_1, ());
                ()
            };
            let v52: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v61: () = {
                Documents::closure8(v10.clone(), string("arguments"), ());
                ()
            };
            let v69: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v77: () = {
                Documents::closure8(v10.clone(), v1_1, ());
                ()
            };
            let v85: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v94: () = {
                Documents::closure8(v10.clone(), string("options"), ());
                ()
            };
            let v102: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v110: () = {
                Documents::closure8(v10.clone(), string("{ "), ());
                ()
            };
            let v119: () = {
                Documents::closure8(v10.clone(), string("command"), ());
                ()
            };
            let v127: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v135: () = {
                Documents::closure8(v10.clone(), v2, ());
                ()
            };
            let v143: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v152: () = {
                Documents::closure8(v10.clone(), string("cancellation_token"), ());
                ()
            };
            let v160: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v165: std::string::String = format!("{:#?}", v3);
            let v198: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v165),
                    (),
                );
                ()
            };
            let v206: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v215: () = {
                Documents::closure8(v10.clone(), string("environment_variables"), ());
                ()
            };
            let v223: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v234: () = {
                Documents::closure8(v10.clone(), sprintf!("{:?}", v4), ());
                ()
            };
            let v242: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v251: () = {
                Documents::closure8(v10.clone(), string("on_line"), ());
                ()
            };
            let v259: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v264: std::string::String = format!("{:#?}", v5);
            let v297: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v264),
                    (),
                );
                ()
            };
            let v305: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v314: () = {
                Documents::closure8(v10.clone(), string("stdin"), ());
                ()
            };
            let v322: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v327: std::string::String = format!("{:#?}", v6);
            let v360: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v327),
                    (),
                );
                ()
            };
            let v368: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v377: () = {
                Documents::closure8(v10.clone(), string("trace"), ());
                ()
            };
            let v385: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v396: () = {
                Documents::closure8(
                    v10.clone(),
                    if v7 { string("true") } else { string("false") },
                    (),
                );
                ()
            };
            let v404: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v413: () = {
                Documents::closure8(v10.clone(), string("working_directory"), ());
                ()
            };
            let v421: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v426: std::string::String = format!("{:#?}", v8);
            let v459: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v426),
                    (),
                );
                ()
            };
            let v468: () = {
                Documents::closure8(v10.clone(), string(" }"), ());
                ()
            };
            let v476: () = {
                Documents::closure8(v10.clone(), string(" }"), ());
                ()
            };
            v10.l0.get().clone()
        }
        pub fn method105(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
            v10: string,
            v11: Option<CancellationToken>,
            v12: Array<(string, string)>,
            v13: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v14: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v15: bool,
            v16: Option<string>,
        ) -> string {
            let v17: string = Documents::method106(v8, v9, v10, v11, v12, v13, v14, v15, v16);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options"),
                v17
            ))
        }
        pub fn closure40(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
            v7: string,
            v8: Vec<std::string::String>,
            unitVar: (),
        ) {
            if Documents::method7(Documents::US0::US0_1) {
                let v13: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v32: Option<i64> = patternInput.5.clone();
                let v31: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v30: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v29: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v28: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v27: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method105(
                    v27.clone(),
                    v28.clone(),
                    v29.clone(),
                    v30.clone(),
                    v31.clone(),
                    v32.clone(),
                    Documents::method8(v27, v28, v29, v30, v31, v32),
                    Documents::method62(),
                    v7,
                    sprintf!("{:?}", v8),
                    v0_1,
                    v1_1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                ))
            };
        }
        pub fn closure41(
            unitVar: (),
            v0_1: Option<std::process::Child>,
        ) -> Option<std::process::Child> {
            v0_1
        }
        pub fn method107() -> Func1<Option<std::process::Child>, Option<std::process::Child>> {
            Func1::new(move |v: Option<std::process::Child>| Documents::closure41((), v))
        }
        pub fn closure42(
            unitVar: (),
            v0_1: std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>,
        ) -> Documents::US23 {
            Documents::US23::US23_0(v0_1)
        }
        pub fn method108(
        ) -> Func1<std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>, Documents::US23>
        {
            Func1::new(
                move |v: std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>| {
                    Documents::closure42((), v)
                },
            )
        }
        pub fn closure43(unitVar: (), v0_1: std::string::String) -> Documents::US23 {
            Documents::US23::US23_1(v0_1)
        }
        pub fn method109() -> Func1<std::string::String, Documents::US23> {
            Func1::new(move |v: std::string::String| Documents::closure43((), v))
        }
        pub fn method110(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
        ) -> string {
            let v9: string = Documents::method76(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options / child error"),
                v9
            ))
        }
        pub fn closure44(v0_1: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_4) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method110(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method74(),
                    v0_1,
                ))
            };
        }
        pub fn method111(
            v0_1: std::sync::MutexGuard<Option<std::process::Child>>,
        ) -> std::sync::MutexGuard<Option<std::process::Child>> {
            v0_1
        }
        pub fn closure45(
            unitVar: (),
            v0_1: Option<std::process::ChildStdin>,
        ) -> Option<std::process::ChildStdin> {
            v0_1
        }
        pub fn method112(
        ) -> Func1<Option<std::process::ChildStdin>, Option<std::process::ChildStdin>> {
            Func1::new(move |v: Option<std::process::ChildStdin>| Documents::closure45((), v))
        }
        pub fn closure46(
            unitVar: (),
            v0_1: std::sync::mpsc::Sender<std::string::String>,
        ) -> std::sync::mpsc::Sender<std::string::String> {
            v0_1
        }
        pub fn method113() -> Func1<
            std::sync::mpsc::Sender<std::string::String>,
            std::sync::mpsc::Sender<std::string::String>,
        > {
            Func1::new(move |v: std::sync::mpsc::Sender<std::string::String>| {
                Documents::closure46((), v)
            })
        }
        pub fn method114() -> Func1<
            std::sync::mpsc::Sender<std::string::String>,
            std::sync::mpsc::Sender<std::string::String>,
        > {
            Func1::new(move |v: std::sync::mpsc::Sender<std::string::String>| {
                Documents::closure46((), v)
            })
        }
        pub fn closure47(
            unitVar: (),
            v0_1: std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
        ) -> std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>> {
            v0_1
        }
        pub fn method115() -> Func1<
            std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
            std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
        > {
            Func1::new(
                move |v: std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>| {
                    Documents::closure47((), v)
                },
            )
        }
        pub fn closure48(unitVar: (), v0_1: std::string::String) -> Documents::US25 {
            Documents::US25::US25_0(v0_1)
        }
        pub fn method116() -> Func1<std::string::String, Documents::US25> {
            Func1::new(move |v: std::string::String| Documents::closure48((), v))
        }
        pub fn closure49(unitVar: (), v0_1: std::string::String) -> Documents::US25 {
            Documents::US25::US25_1(v0_1)
        }
        pub fn method117() -> Func1<std::string::String, Documents::US25> {
            Func1::new(move |v: std::string::String| Documents::closure49((), v))
        }
        pub fn method119(v0_1: bool, v1_1: std::string::String) -> string {
            let v3: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v10: () = {
                Documents::closure8(v3.clone(), string("{ "), ());
                ()
            };
            let v19: () = {
                Documents::closure8(v3.clone(), string("trace\'"), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v39: () = {
                Documents::closure8(
                    v3.clone(),
                    if v0_1 {
                        string("true")
                    } else {
                        string("false")
                    },
                    (),
                );
                ()
            };
            let v48: () = {
                Documents::closure8(v3.clone(), string("; "), ());
                ()
            };
            let v57: () = {
                Documents::closure8(v3.clone(), string("e"), ());
                ()
            };
            let v65: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v70: std::string::String = format!("{:#?}", v1_1);
            let v103: () = {
                Documents::closure8(v3.clone(), fable_library_rust::String_::fromString(v70), ());
                ()
            };
            let v112: () = {
                Documents::closure8(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method118(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: bool,
            v9: std::string::String,
        ) -> string {
            let v10: string = Documents::method119(v8, v9);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.stdio_line"),
                v10
            ))
        }
        pub fn closure50(v0_1: bool, v1_1: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_4) {
                let v6: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v25: Option<i64> = patternInput.5.clone();
                let v24: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v23: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v22: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v21: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v20: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method118(
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    Documents::method8(v20, v21, v22, v23, v24, v25),
                    Documents::method74(),
                    v0_1,
                    v1_1,
                ))
            };
        }
        pub fn method120() -> string {
            let v6: string = Documents::method13(getCharAt(toLower(string("Verbose")), 0_i32));
            let v9: &str = inline_colorization::color_bright_black;
            let v12: &str = &*v6;
            let v35: &str = inline_colorization::color_reset;
            let v37: std::string::String = format!("{}{}{}", v9, v12, v35);
            fable_library_rust::String_::fromString(v37)
        }
        pub fn method122() -> string {
            let v1_1: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            v1_1.l0.get().clone()
        }
        pub fn method121(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
        ) -> string {
            let v9: string = Documents::method122();
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                v8,
                v9
            ))
        }
        pub fn closure51(v0_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_0) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(if (v0_1.clone()) == string("") {
                    string("")
                } else {
                    Documents::method121(
                        v19.clone(),
                        v20.clone(),
                        v21.clone(),
                        v22.clone(),
                        v23.clone(),
                        v24.clone(),
                        Documents::method8(v19, v20, v21, v22, v23, v24),
                        Documents::method120(),
                        v0_1,
                    )
                })
            };
        }
        pub fn closure52(
            unitVar: (),
            v0_1: std::sync::mpsc::SendError<std::string::String>,
        ) -> std::string::String {
            format!("{}", v0_1)
        }
        pub fn method123(
        ) -> Func1<std::sync::mpsc::SendError<std::string::String>, std::string::String> {
            Func1::new(move |v: std::sync::mpsc::SendError<std::string::String>| {
                Documents::closure52((), v)
            })
        }
        pub fn method124(v0_1: Result<(), string>) -> Result<(), string> {
            v0_1
        }
        pub fn closure53(
            unitVar: (),
            v0_1: Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>,
        ) -> Documents::US26 {
            Documents::US26::US26_0(v0_1)
        }
        pub fn method125() -> Func1<
            Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>,
            Documents::US26,
        > {
            Func1::new(
                move |v: Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>| {
                    Documents::closure53((), v)
                },
            )
        }
        pub fn method126(
            v0_1: std::sync::MutexGuard<Option<std::process::ChildStdin>>,
        ) -> std::sync::MutexGuard<Option<std::process::ChildStdin>> {
            v0_1
        }
        pub fn closure54(
            unitVar: (),
            v0_1: std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>,
        ) -> Documents::US27 {
            Documents::US27::US27_0(v0_1)
        }
        pub fn method127(
        ) -> Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, Documents::US27>
        {
            Func1::new(
                move |v: std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>| {
                    Documents::closure54((), v)
                },
            )
        }
        pub fn method128(
            v0_1: std::sync::MutexGuard<std::process::ChildStdin>,
        ) -> std::sync::MutexGuard<std::process::ChildStdin> {
            v0_1
        }
        pub fn method129(
            v0_1: std::thread::JoinHandle<Result<(), string>>,
        ) -> std::thread::JoinHandle<Result<(), string>> {
            v0_1
        }
        pub fn closure55(unitVar: (), v0_1: std::process::Output) -> Documents::US28 {
            Documents::US28::US28_0(v0_1)
        }
        pub fn method130() -> Func1<std::process::Output, Documents::US28> {
            Func1::new(move |v: std::process::Output| Documents::closure55((), v))
        }
        pub fn closure56(unitVar: (), v0_1: std::string::String) -> Documents::US28 {
            Documents::US28::US28_1(v0_1)
        }
        pub fn method131() -> Func1<std::string::String, Documents::US28> {
            Func1::new(move |v: std::string::String| Documents::closure56((), v))
        }
        pub fn method132(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
        ) -> string {
            let v9: string = Documents::method76(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options / output error"),
                v9
            ))
        }
        pub fn closure57(v0_1: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_4) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method132(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method74(),
                    v0_1,
                ))
            };
        }
        pub fn closure58(unitVar: (), v0_1: i32) -> Documents::US29 {
            Documents::US29::US29_0(v0_1)
        }
        pub fn method133() -> Func1<i32, Documents::US29> {
            Func1::new(move |v: i32| Documents::closure58((), v))
        }
        pub fn method134() -> string {
            string("\n")
        }
        pub fn method136(v0_1: i32, v1_1: i32) -> string {
            let v3: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v10: () = {
                Documents::closure8(v3.clone(), string("{ "), ());
                ()
            };
            let v19: () = {
                Documents::closure8(v3.clone(), string("exit_code"), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v36: () = {
                Documents::closure8(v3.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v45: () = {
                Documents::closure8(v3.clone(), string("; "), ());
                ()
            };
            let v54: () = {
                Documents::closure8(v3.clone(), string("std_trace_length"), ());
                ()
            };
            let v62: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v70: () = {
                Documents::closure8(v3.clone(), sprintf!("{}", v1_1), ());
                ()
            };
            let v79: () = {
                Documents::closure8(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method135(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: i32,
        ) -> string {
            let v10: string = Documents::method136(v8, v9);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options / result"),
                v10
            ))
        }
        pub fn closure59(v0_1: i32, v1_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_0) {
                let v6: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v25: Option<i64> = patternInput.5.clone();
                let v24: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v23: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v22: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v21: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v20: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method135(
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    Documents::method8(v20, v21, v22, v23, v24, v25),
                    Documents::method120(),
                    v0_1,
                    length(v1_1),
                ))
            };
        }
        pub fn method140(
            v0_1: string,
            v1_1: Documents::US5,
            v2: string,
            v3: Option<CancellationToken>,
            v4: Array<(string, string)>,
            v5: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v6: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v7: bool,
            v8: Option<string>,
        ) -> string {
            let v10: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v17: () = {
                Documents::closure8(v10.clone(), string("{ "), ());
                ()
            };
            let v26: () = {
                Documents::closure8(v10.clone(), string("file_name"), ());
                ()
            };
            let v35: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v43: () = {
                Documents::closure8(v10.clone(), v0_1, ());
                ()
            };
            let v52: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v61: () = {
                Documents::closure8(v10.clone(), string("arguments"), ());
                ()
            };
            let v69: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v80: () = {
                Documents::closure8(v10.clone(), sprintf!("{:?}", v1_1), ());
                ()
            };
            let v88: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v97: () = {
                Documents::closure8(v10.clone(), string("options"), ());
                ()
            };
            let v105: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v113: () = {
                Documents::closure8(v10.clone(), string("{ "), ());
                ()
            };
            let v122: () = {
                Documents::closure8(v10.clone(), string("command"), ());
                ()
            };
            let v130: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v138: () = {
                Documents::closure8(v10.clone(), v2, ());
                ()
            };
            let v146: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v155: () = {
                Documents::closure8(v10.clone(), string("cancellation_token"), ());
                ()
            };
            let v163: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v168: std::string::String = format!("{:#?}", v3);
            let v201: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v168),
                    (),
                );
                ()
            };
            let v209: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v218: () = {
                Documents::closure8(v10.clone(), string("environment_variables"), ());
                ()
            };
            let v226: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v237: () = {
                Documents::closure8(v10.clone(), sprintf!("{:?}", v4), ());
                ()
            };
            let v245: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v254: () = {
                Documents::closure8(v10.clone(), string("on_line"), ());
                ()
            };
            let v262: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v267: std::string::String = format!("{:#?}", v5);
            let v300: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v267),
                    (),
                );
                ()
            };
            let v308: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v317: () = {
                Documents::closure8(v10.clone(), string("stdin"), ());
                ()
            };
            let v325: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v330: std::string::String = format!("{:#?}", v6);
            let v363: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v330),
                    (),
                );
                ()
            };
            let v371: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v380: () = {
                Documents::closure8(v10.clone(), string("trace"), ());
                ()
            };
            let v388: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v399: () = {
                Documents::closure8(
                    v10.clone(),
                    if v7 { string("true") } else { string("false") },
                    (),
                );
                ()
            };
            let v407: () = {
                Documents::closure8(v10.clone(), string("; "), ());
                ()
            };
            let v416: () = {
                Documents::closure8(v10.clone(), string("working_directory"), ());
                ()
            };
            let v424: () = {
                Documents::closure8(v10.clone(), string(" = "), ());
                ()
            };
            let v429: std::string::String = format!("{:#?}", v8);
            let v462: () = {
                Documents::closure8(
                    v10.clone(),
                    fable_library_rust::String_::fromString(v429),
                    (),
                );
                ()
            };
            let v471: () = {
                Documents::closure8(v10.clone(), string(" }"), ());
                ()
            };
            let v479: () = {
                Documents::closure8(v10.clone(), string(" }"), ());
                ()
            };
            v10.l0.get().clone()
        }
        pub fn method139(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: Documents::US5,
            v10: string,
            v11: Option<CancellationToken>,
            v12: Array<(string, string)>,
            v13: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v14: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v15: bool,
            v16: Option<string>,
        ) -> string {
            let v17: string = Documents::method140(v8, v9, v10, v11, v12, v13, v14, v15, v16);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options_async"),
                v17
            ))
        }
        pub fn closure60(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
            v7: Documents::US5,
            v8: string,
            unitVar: (),
        ) {
            if Documents::method7(Documents::US0::US0_1) {
                let v13: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v32: Option<i64> = patternInput.5.clone();
                let v31: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v30: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v29: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v28: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v27: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method139(
                    v27.clone(),
                    v28.clone(),
                    v29.clone(),
                    v30.clone(),
                    v31.clone(),
                    v32.clone(),
                    Documents::method8(v27, v28, v29, v30, v31, v32),
                    Documents::method62(),
                    v8,
                    v7,
                    v0_1,
                    v1_1,
                    v2,
                    v3,
                    v4,
                    v5,
                    v6,
                ))
            };
        }
        pub fn method141(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
        ) -> Array<(string, string)> {
            v2
        }
        pub fn method144(v0_1: ()) -> string {
            unbox::<string>(&getZero())
        }
        pub fn closure62(
            unitVar: (),
            v0_1: Func1<(i32, string, bool), Arc<Async<()>>>,
        ) -> Documents::US30 {
            Documents::US30::US30_0(v0_1)
        }
        pub fn method145() -> Func1<Func1<(i32, string, bool), Arc<Async<()>>>, Documents::US30> {
            Func1::new(move |v: Func1<(i32, string, bool), Arc<Async<()>>>| {
                Documents::closure62((), v)
            })
        }
        pub fn method146(v0_1: ()) -> i32 {
            unbox::<i32>(&getZero())
        }
        pub fn closure63(v0_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_0) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(if (v0_1.clone()) == string("") {
                    string("")
                } else {
                    Documents::method121(
                        v19.clone(),
                        v20.clone(),
                        v21.clone(),
                        v22.clone(),
                        v23.clone(),
                        v24.clone(),
                        Documents::method8(v19, v20, v21, v22, v23, v24),
                        Documents::method120(),
                        v0_1,
                    )
                })
            };
        }
        pub fn method143(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
            v7: (),
            v8: LrcPtr<ConcurrentStack_1<string>>,
            v9: bool,
            v10: (),
        ) -> Arc<Async<()>> {
            getZero()
        }
        pub fn method142(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
            v7: (),
            v8: LrcPtr<ConcurrentStack_1<string>>,
            v9: bool,
            v10: (),
        ) -> Arc<Async<()>> {
            Documents::method143(v0_1, v1_1, v2, v3, v4, v5, v6, v7, v8, v9, v10)
        }
        pub fn closure61(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
            v7: (),
            v8: LrcPtr<ConcurrentStack_1<string>>,
            v9: bool,
            v10: (),
        ) {
            let v11: Arc<Async<()>> =
                Documents::method142(v0_1, v1_1, v2, v3, v4, v5, v6, v7, v8, v9, v10);
            getZero::<()>();
            ()
        }
        pub fn closure64(unitVar: (), v0_1: CancellationToken) -> Documents::US31 {
            Documents::US31::US31_0(v0_1)
        }
        pub fn method147() -> Func1<CancellationToken, Documents::US31> {
            Func1::new(move |v: CancellationToken| Documents::closure64((), v))
        }
        pub fn method148(v0_1: CancellationToken) -> Arc<Async<CancellationToken>> {
            getZero()
        }
        pub fn method149(v0_1: ()) -> bool {
            unbox::<bool>(&getZero())
        }
        pub fn method150(v0_1: ()) {
            ();
        }
        pub fn closure65(v0_1: (), unitVar: ()) {
            if (Documents::method149(v0_1)) == false {
                Documents::method150(v0_1);
            };
        }
        pub fn closure66(v0_1: LrcPtr<Exception>, unitVar: ()) -> LrcPtr<Exception> {
            v0_1
        }
        pub fn method153(v0_1: LrcPtr<TaskCanceledException>) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("ex"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v32: std::string::String = format!("{:#?}", v0_1);
            let v65: () = {
                Documents::closure8(v2.clone(), fable_library_rust::String_::fromString(v32), ());
                ()
            };
            let v74: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method152(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: LrcPtr<TaskCanceledException>,
        ) -> string {
            let v9: string = Documents::method153(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options_async / WaitForExitAsync"),
                v9
            ))
        }
        pub fn closure67(v0_1: LrcPtr<TaskCanceledException>, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_3) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method152(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method31(),
                    v0_1,
                ))
            };
        }
        pub fn method151(
            v0_1: (),
            v1_1: LrcPtr<ConcurrentStack_1<string>>,
            v2: CancellationToken,
        ) -> Arc<Async<i32>> {
            getZero()
        }
        pub fn method155(v0_1: i32, v1_1: i32) -> string {
            let v3: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v10: () = {
                Documents::closure8(v3.clone(), string("{ "), ());
                ()
            };
            let v19: () = {
                Documents::closure8(v3.clone(), string("exit_code"), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v36: () = {
                Documents::closure8(v3.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v45: () = {
                Documents::closure8(v3.clone(), string("; "), ());
                ()
            };
            let v54: () = {
                Documents::closure8(v3.clone(), string("output_length"), ());
                ()
            };
            let v62: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v70: () = {
                Documents::closure8(v3.clone(), sprintf!("{}", v1_1), ());
                ()
            };
            let v79: () = {
                Documents::closure8(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method154(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: i32,
        ) -> string {
            let v10: string = Documents::method155(v8, v9);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("runtime.execute_with_options_async"),
                v10
            ))
        }
        pub fn closure68(v0_1: i32, v1_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_1) {
                let v6: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v25: Option<i64> = patternInput.5.clone();
                let v24: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v23: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v22: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v21: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v20: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method154(
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    Documents::method8(v20, v21, v22, v23, v24, v25),
                    Documents::method62(),
                    v0_1,
                    length(v1_1),
                ))
            };
        }
        pub fn method138(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
        ) -> Arc<Async<(i32, string)>> {
            getZero()
        }
        pub fn method137(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
        ) -> Arc<Async<(i32, string)>> {
            Documents::method138(v0_1, v1_1, v2, v3, v4, v5, v6)
        }
        pub fn method81(
            v0_1: string,
            v1_1: Option<CancellationToken>,
            v2: Array<(string, string)>,
            v3: Option<Func1<(i32, string, bool), Arc<Async<()>>>>,
            v4: Option<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
            v5: bool,
            v6: Option<string>,
        ) -> (i32, string) {
            let v9: Documents::US13 = Documents::method83(Documents::method82(
                v0_1.clone(),
                v1_1.clone(),
                v2.clone(),
                v3.clone(),
                v4.clone(),
                v5,
                v6.clone(),
            ));
            let patternInput: (string, Documents::US5) = match &v9 {
                Documents::US13::US13_0(v9_0_0, v9_0_1) => (v9_0_0.clone(), v9_0_1.clone()),
                Documents::US13::US13_1(v9_1_0) => panic!(
                    "{}",
                    concat(new_array(&[
                        string("resultm.get / Result value was Error: "),
                        v9_1_0.clone()
                    ])),
                ),
            };
            let v21: Documents::US5 = patternInput.1.clone();
            let v20: string = patternInput.0.clone();
            let v26: Documents::US21 = Documents::method95(match &v21 {
                Documents::US5::US5_0(v21_0_0) => match &v21 {
                    Documents::US5::US5_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => string(""),
            });
            let v34: Array<string> = match &v26 {
                Documents::US21::US21_0(v26_0_0) => v26_0_0.clone(),
                Documents::US21::US21_1(v26_1_0) => panic!(
                    "{}",
                    concat(new_array(&[
                        string("resultm.get / Result value was Error: "),
                        v26_1_0.clone()
                    ])),
                ),
            };
            let v36: Vec<string> = v34.to_vec();
            let v38: bool = true;
            let _vec_map: Vec<_> = v36
                .into_iter()
                .map(|x| {
                    //;
                    let v40: string = x;
                    let v43: &str = &*v40;
                    let v67: std::string::String = String::from(v43);
                    let v90: bool = true;
                    v67
                })
                .collect::<Vec<_>>();
            let v92: Vec<std::string::String> = _vec_map;
            let v95: () = {
                Documents::closure40(
                    v0_1,
                    v1_1,
                    v2.clone(),
                    v3,
                    v4.clone(),
                    v5,
                    v6.clone(),
                    v20.clone(),
                    v92.clone(),
                    (),
                );
                ()
            };
            let _v138: MutCell<Option<LrcPtr<(i32, string)>>> =
                MutCell::new(None::<LrcPtr<(i32, string)>>);
            {
                let x_3: LrcPtr<(i32, string)> = (Func0::new({
                    let v2 = v2.clone();
                    let v20 = v20.clone();
                    let v4 = v4.clone();
                    let v5 = v5.clone();
                    let v6 = v6.clone();
                    let v92 = v92.clone();
                    move || {
                        let v140: std::process::Command = std::process::Command::new(&*v20.clone());
                        let v142: bool = true;
                        let mut v140 = v140;
                        let v144: bool = true;
                        std::process::Command::args(&mut v140, &*v92.clone());
                        let v146: std::process::Command = v140;
                        let v148: std::process::Stdio = std::process::Stdio::piped();
                        let v150: bool = true;
                        let mut v146 = v146;
                        let v152: bool = true;
                        std::process::Command::stdout(&mut v146, std::process::Stdio::piped());
                        let v154: std::process::Command = v146;
                        let v156: std::process::Stdio = std::process::Stdio::piped();
                        let v158: bool = true;
                        let mut v154 = v154;
                        let v160: bool = true;
                        std::process::Command::stderr(&mut v154, std::process::Stdio::piped());
                        let v162: std::process::Command = v154;
                        let v164: std::process::Stdio = std::process::Stdio::piped();
                        let v166: bool = true;
                        let mut v162 = v162;
                        let v168: bool = true;
                        std::process::Command::stdin(&mut v162, std::process::Stdio::piped());
                        let v170: std::process::Command = v162;
                        let v184: Documents::US5 = defaultValue(
                            Documents::US5::US5_1,
                            map(Documents::method6(), v6.clone()),
                        );
                        let v198: std::process::Command = match &v184 {
                            Documents::US5::US5_0(v184_0_0) => {
                                let v190: bool = true;
                                let mut v170 = v170;
                                let v192: bool = true;
                                std::process::Command::current_dir(
                                    &mut v170,
                                    &*match &v184 {
                                        Documents::US5::US5_0(x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone(),
                                );
                                v170
                            }
                            _ => v170,
                        };
                        let v220: std::process::Command = if (get_Count(v2.clone()) as u64) == 0_u64
                        {
                            v198
                        } else {
                            let v202: Vec<(string, string)> = v2.clone().to_vec();
                            let v204: bool = true;
                            let _vec_fold_ = v202.into_iter().fold(v198, |acc, x| {
                                //;
                                let v206: std::process::Command = acc;
                                let patternInput_1: (string, string) = x;
                                let v211: bool = true;
                                let mut v206 = v206;
                                let v213: bool = true;
                                std::process::Command::env(
                                    &mut v206,
                                    &*patternInput_1.0.clone(),
                                    &*patternInput_1.1.clone(),
                                );
                                let v215: std::process::Command = v206;
                                let v217: bool = true;
                                v215
                            });
                            _vec_fold_
                        };
                        let v222: bool = true;
                        let mut v220 = v220;
                        let v224: Result<std::process::Child, std::io::Error> =
                            std::process::Command::spawn(&mut v220);
                        let v225 = Documents::method65();
                        let v228: Result<std::process::Child, std::string::String> =
                            v224.map_err(|x| v225(x));
                        let v240 = Documents::method107();
                        let v242: bool = true;
                        let _result_map_ = v228.map(|x| {
                            //;
                            let v248: Option<std::process::Child> = v240(Some(x));
                            let v250: std::sync::Mutex<Option<std::process::Child>> =
                                std::sync::Mutex::new(v248);
                            let v252: std::sync::Arc<
                                std::sync::Mutex<Option<std::process::Child>>,
                            > = std::sync::Arc::new(v250);
                            let v254: bool = true;
                            v252
                        });
                        let v256: Result<
                            std::sync::Arc<std::sync::Mutex<Option<std::process::Child>>>,
                            std::string::String,
                        > = _result_map_;
                        let v257 = Documents::method108();
                        let v258 = Documents::method109();
                        let v260: Documents::US23 = match v256 {
                            Ok(x) => v257(x),
                            Err(e) => v258(e),
                        };
                        let patternInput_3: (i32, Documents::US6, Documents::US24) = match &v260 {
                            Documents::US23::US23_0(v260_0_0) => {
                                let v261: std::sync::Arc<
                                    std::sync::Mutex<Option<std::process::Child>>,
                                > = v260_0_0.clone();
                                let v263: bool = true;
                                let _capture = (|| {
                                    //;
                                    let v265: std::sync::Arc<
                                        std::sync::Mutex<Option<std::process::Child>>,
                                    > = v261.clone();
                                    let v267: Result<
                                        std::sync::MutexGuard<Option<std::process::Child>>,
                                        std::sync::PoisonError<
                                            std::sync::MutexGuard<Option<std::process::Child>>,
                                        >,
                                    > = v265.lock();
                                    let v282: std::sync::MutexGuard<Option<std::process::Child>> =
                                        Documents::method111(v267.unwrap());
                                    let v284: bool = true;
                                    let mut v282 = v282;
                                    let v286: &mut Option<std::process::Child> = &mut v282;
                                    let v288: Option<&mut std::process::Child> = v286.as_mut();
                                    let v290: &mut std::process::Child = v288.unwrap();
                                    let v292: &mut Option<std::process::ChildStdout> =
                                        &mut v290.stdout;
                                    let v294: Option<std::process::ChildStdout> =
                                        Option::take(v292);
                                    let v296: std::process::ChildStdout = v294.unwrap();
                                    let v298: bool = true;
                                    v296
                                })();
                                let v300: std::process::ChildStdout = _capture;
                                let v302: bool = true;
                                let _capture = (|| {
                                    //;
                                    let v304: std::sync::Arc<
                                        std::sync::Mutex<Option<std::process::Child>>,
                                    > = v261.clone();
                                    let v306: Result<
                                        std::sync::MutexGuard<Option<std::process::Child>>,
                                        std::sync::PoisonError<
                                            std::sync::MutexGuard<Option<std::process::Child>>,
                                        >,
                                    > = v304.lock();
                                    let v321: std::sync::MutexGuard<Option<std::process::Child>> =
                                        Documents::method111(v306.unwrap());
                                    let v323: bool = true;
                                    let mut v321 = v321;
                                    let v325: &mut Option<std::process::Child> = &mut v321;
                                    let v327: Option<&mut std::process::Child> = v325.as_mut();
                                    let v329: &mut std::process::Child = v327.unwrap();
                                    let v331: &mut Option<std::process::ChildStderr> =
                                        &mut v329.stderr;
                                    let v333: Option<std::process::ChildStderr> =
                                        Option::take(v331);
                                    let v335: std::process::ChildStderr = v333.unwrap();
                                    let v337: bool = true;
                                    v335
                                })();
                                let v339: std::process::ChildStderr = _capture;
                                let v341: bool = true;
                                let _capture = (|| {
                                    //;
                                    let v343: std::sync::Arc<
                                        std::sync::Mutex<Option<std::process::Child>>,
                                    > = v261.clone();
                                    let v345: Result<
                                        std::sync::MutexGuard<Option<std::process::Child>>,
                                        std::sync::PoisonError<
                                            std::sync::MutexGuard<Option<std::process::Child>>,
                                        >,
                                    > = v343.lock();
                                    let v360: std::sync::MutexGuard<Option<std::process::Child>> =
                                        Documents::method111(v345.unwrap());
                                    let v362: bool = true;
                                    let mut v360 = v360;
                                    let v364: &mut Option<std::process::Child> = &mut v360;
                                    let v366: Option<&mut std::process::Child> = v364.as_mut();
                                    let v368: &mut std::process::Child = v366.unwrap();
                                    let v370: &mut Option<std::process::ChildStdin> =
                                        &mut v368.stdin;
                                    let v372: Option<std::process::ChildStdin> = Option::take(v370);
                                    let v374: std::process::ChildStdin = v372.unwrap();
                                    let v379: Option<std::process::ChildStdin> =
                                        (Documents::method112())(Some(v374));
                                    let v381: std::sync::Mutex<Option<std::process::ChildStdin>> =
                                        std::sync::Mutex::new(v379);
                                    let v383: std::sync::Arc<
                                        std::sync::Mutex<Option<std::process::ChildStdin>>,
                                    > = std::sync::Arc::new(v381);
                                    let v385: bool = true;
                                    v383
                                })();
                                let v387: std::sync::Arc<
                                    std::sync::Mutex<Option<std::process::ChildStdin>>,
                                > = _capture;
                                let patternInput_2: (
                                    std::sync::mpsc::Sender<std::string::String>,
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                ) = {
                                    let (sender, receiver) = std::sync::mpsc::channel();
                                    (sender, std::sync::Arc::new(receiver))
                                };
                                let v389: std::sync::mpsc::Sender<std::string::String> =
                                    patternInput_2.0.clone();
                                let v392: std::sync::mpsc::Sender<std::string::String> =
                                    (Documents::method113())(v389.clone());
                                let v394: std::sync::Mutex<
                                    std::sync::mpsc::Sender<std::string::String>,
                                > = std::sync::Mutex::new(v392);
                                let v396: std::sync::Arc<
                                    std::sync::Mutex<std::sync::mpsc::Sender<std::string::String>>,
                                > = std::sync::Arc::new(v394);
                                let v398: std::sync::mpsc::Sender<std::string::String> =
                                    (Documents::method114())(v389);
                                let v400: std::sync::Mutex<
                                    std::sync::mpsc::Sender<std::string::String>,
                                > = std::sync::Mutex::new(v398);
                                let v402: std::sync::Arc<
                                    std::sync::Mutex<std::sync::mpsc::Sender<std::string::String>>,
                                > = std::sync::Arc::new(v400);
                                let v404: std::sync::Arc<
                                    std::sync::mpsc::Receiver<std::string::String>,
                                > = (Documents::method115())(patternInput_2.1.clone());
                                let v406: std::sync::Mutex<
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                > = std::sync::Mutex::new(v404);
                                let v408: std::sync::Arc<
                                    std::sync::Mutex<
                                        std::sync::Arc<
                                            std::sync::mpsc::Receiver<std::string::String>,
                                        >,
                                    >,
                                > = std::sync::Arc::new(v406);
                                let v410: bool = true;
                                let __spawn = std::thread::spawn(move || {
                                    //;
                                    let v412: encoding_rs_io::DecodeReaderBytes<
                                        std::process::ChildStdout,
                                        Vec<u8>,
                                    > = encoding_rs_io::DecodeReaderBytesBuilder::new()
                                        .utf8_passthru(true)
                                        .build(v300);
                                    let v414: std::io::BufReader<
                                        encoding_rs_io::DecodeReaderBytes<
                                            std::process::ChildStdout,
                                            Vec<u8>,
                                        >,
                                    > = std::io::BufReader::new(v412);
                                    let v416: std::io::Lines<
                                        std::io::BufReader<
                                            encoding_rs_io::DecodeReaderBytes<
                                                std::process::ChildStdout,
                                                Vec<u8>,
                                            >,
                                        >,
                                    > = std::io::BufRead::lines(v414);
                                    let v418: bool = true;
                                    let mut v416 = v416;
                                    let _iter_try_for_each = v416.try_for_each(|x| {
                                        //;
                                        let v420: Result<std::string::String, std::io::Error> = x;
                                        let v422: std::sync::Arc<
                                            std::sync::Mutex<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                        > = v396.clone();
                                        let v423 = Documents::method65();
                                        let v426: Result<std::string::String, std::string::String> =
                                            v420.map_err(|x| v423(x));
                                        let v438 = Documents::method116();
                                        let v439 = Documents::method117();
                                        let v441: Documents::US25 = match v426 {
                                            Ok(x) => v438(x),
                                            Err(e) => v439(e),
                                        };
                                        let v636: std::string::String = match &v441 {
                                            Documents::US25::US25_0(v441_0_0) => {
                                                let v444: string =
                                                    fable_library_rust::String_::fromString(
                                                        v441_0_0.clone(),
                                                    );
                                                let v446: &encoding_rs::Encoding =
                                                    encoding_rs::UTF_8;
                                                let v448: std::borrow::Cow<[u8]> =
                                                    v446.encode(&*v444).0;
                                                let v450: &[u8] = v448.as_ref();
                                                let v452: Result<&str, std::str::Utf8Error> =
                                                    std::str::from_utf8(v450);
                                                let v455: &str = v452.unwrap();
                                                let v486: std::string::String = String::from(v455);
                                                let v493: string = concat(new_array(&[
                                                    string("> "),
                                                    fable_library_rust::String_::fromString(
                                                        v486.clone(),
                                                    ),
                                                ]));
                                                if v5 {
                                                    let v496: () = {
                                                        Documents::closure51(v493.clone(), ());
                                                        ()
                                                    };
                                                    ()
                                                } else {
                                                    let v541: () = {
                                                        Documents::closure11(v493, ());
                                                        ()
                                                    };
                                                    ()
                                                }
                                                v486
                                            }
                                            Documents::US25::US25_1(v441_1_0) => {
                                                let v543: std::string::String = v441_1_0.clone();
                                                let v546: () = {
                                                    Documents::closure50(v5, v543.clone(), ());
                                                    ()
                                                };
                                                let v586: string =
                                                    sprintf!("\u{001b}[4;7m{}\u{001b}[0m", v543);
                                                let v589: &str = &*v586;
                                                String::from(v589)
                                            }
                                        };
                                        let v638: std::sync::Arc<
                                            std::sync::Mutex<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                        > = v422;
                                        let v640: Result<
                                            std::sync::MutexGuard<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                            std::sync::PoisonError<
                                                std::sync::MutexGuard<
                                                    std::sync::mpsc::Sender<std::string::String>,
                                                >,
                                            >,
                                        > = v638.lock();
                                        let v643: std::sync::MutexGuard<
                                            std::sync::mpsc::Sender<std::string::String>,
                                        > = v640.unwrap();
                                        let v656: &std::sync::mpsc::Sender<std::string::String> =
                                            &v643;
                                        let v658: Result<
                                            (),
                                            std::sync::mpsc::SendError<std::string::String>,
                                        > = v656.send(v636);
                                        let v659 = Documents::method123();
                                        let v662: Result<(), std::string::String> =
                                            v658.map_err(|x| v659(x));
                                        let v675: _ = v662;
                                        let v677: bool = true;
                                        v675
                                    }); //;
                                    let v680: Result<(), string> = Documents::method124(
                                        _iter_try_for_each.map_err(|x| x.into()),
                                    );
                                    let v683: string = string("}");
                                    let v687: bool = true;
                                    let v684 = v680;
                                    let v699: string = append(
                                        (append(
                                            (append(
                                                (append(string("true; v684 "), (v683))),
                                                string("); "),
                                            )),
                                            string(""),
                                        )),
                                        string(" // rust.fix_closure\'"),
                                    );
                                    let v700: bool = true;
                                    v684
                                }); // rust.fix_closure';
                                let v702: std::thread::JoinHandle<Result<(), string>> = __spawn;
                                let v704: bool = true;
                                let __spawn = std::thread::spawn(move || {
                                    //;
                                    let v706: encoding_rs_io::DecodeReaderBytes<
                                        std::process::ChildStderr,
                                        Vec<u8>,
                                    > = encoding_rs_io::DecodeReaderBytesBuilder::new()
                                        .utf8_passthru(true)
                                        .build(v339);
                                    let v708: std::io::BufReader<
                                        encoding_rs_io::DecodeReaderBytes<
                                            std::process::ChildStderr,
                                            Vec<u8>,
                                        >,
                                    > = std::io::BufReader::new(v706);
                                    let v710: std::io::Lines<
                                        std::io::BufReader<
                                            encoding_rs_io::DecodeReaderBytes<
                                                std::process::ChildStderr,
                                                Vec<u8>,
                                            >,
                                        >,
                                    > = std::io::BufRead::lines(v708);
                                    let v712: bool = true;
                                    let mut v710 = v710;
                                    let _iter_try_for_each = v710.try_for_each(|x| {
                                        //;
                                        let v714: Result<std::string::String, std::io::Error> = x;
                                        let v716: std::sync::Arc<
                                            std::sync::Mutex<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                        > = v402.clone();
                                        let v717 = Documents::method65();
                                        let v720: Result<std::string::String, std::string::String> =
                                            v714.map_err(|x| v717(x));
                                        let v732 = Documents::method116();
                                        let v733 = Documents::method117();
                                        let v735: Documents::US25 = match v720 {
                                            Ok(x) => v732(x),
                                            Err(e) => v733(e),
                                        };
                                        let v978: std::string::String = match &v735 {
                                            Documents::US25::US25_0(v735_0_0) => {
                                                let v738: string =
                                                    fable_library_rust::String_::fromString(
                                                        v735_0_0.clone(),
                                                    );
                                                let v740: &encoding_rs::Encoding =
                                                    encoding_rs::UTF_8;
                                                let v742: std::borrow::Cow<[u8]> =
                                                    v740.encode(&*v738).0;
                                                let v744: &[u8] = v742.as_ref();
                                                let v746: Result<&str, std::str::Utf8Error> =
                                                    std::str::from_utf8(v744);
                                                let v749: &str = v746.unwrap();
                                                let v780: std::string::String = String::from(v749);
                                                let v787: string = concat(new_array(&[
                                                    string("! "),
                                                    fable_library_rust::String_::fromString(
                                                        v780.clone(),
                                                    ),
                                                ]));
                                                if v5 {
                                                    let v790: () = {
                                                        Documents::closure51(v787.clone(), ());
                                                        ()
                                                    };
                                                    ()
                                                } else {
                                                    let v834: () = {
                                                        Documents::closure11(v787, ());
                                                        ()
                                                    };
                                                    ()
                                                }
                                                {
                                                    let v836: string = sprintf!(
                                                        "\u{001b}[4;7m{}\u{001b}[0m",
                                                        v780
                                                    );
                                                    let v839: &str = &*v836;
                                                    String::from(v839)
                                                }
                                            }
                                            Documents::US25::US25_1(v735_1_0) => {
                                                let v885: std::string::String = v735_1_0.clone();
                                                let v888: () = {
                                                    Documents::closure50(v5, v885.clone(), ());
                                                    ()
                                                };
                                                let v928: string =
                                                    sprintf!("\u{001b}[4;7m{}\u{001b}[0m", v885);
                                                let v931: &str = &*v928;
                                                String::from(v931)
                                            }
                                        };
                                        let v980: std::sync::Arc<
                                            std::sync::Mutex<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                        > = v716;
                                        let v982: Result<
                                            std::sync::MutexGuard<
                                                std::sync::mpsc::Sender<std::string::String>,
                                            >,
                                            std::sync::PoisonError<
                                                std::sync::MutexGuard<
                                                    std::sync::mpsc::Sender<std::string::String>,
                                                >,
                                            >,
                                        > = v980.lock();
                                        let v985: std::sync::MutexGuard<
                                            std::sync::mpsc::Sender<std::string::String>,
                                        > = v982.unwrap();
                                        let v998: &std::sync::mpsc::Sender<std::string::String> =
                                            &v985;
                                        let v1000: Result<
                                            (),
                                            std::sync::mpsc::SendError<std::string::String>,
                                        > = v998.send(v978);
                                        let v1001 = Documents::method123();
                                        let v1004: Result<(), std::string::String> =
                                            v1000.map_err(|x| v1001(x));
                                        let v1017: _ = v1004;
                                        let v1019: bool = true;
                                        v1017
                                    }); //;
                                    let v1022: Result<(), string> = Documents::method124(
                                        _iter_try_for_each.map_err(|x| x.into()),
                                    );
                                    let v1023: string = string("}");
                                    let v1027: bool = true;
                                    let v1024 = v1022;
                                    let v1039: string = append(
                                        (append(
                                            (append(
                                                (append(string("true; v1024 "), (v1023))),
                                                string("); "),
                                            )),
                                            string(""),
                                        )),
                                        string(" // rust.fix_closure\'"),
                                    );
                                    let v1040: bool = true;
                                    v1024
                                }); // rust.fix_closure';
                                let v1042: std::thread::JoinHandle<Result<(), string>> = __spawn;
                                let v1056: Documents::US26 = defaultValue(
                                    Documents::US26::US26_1,
                                    map(Documents::method125(), v4.clone()),
                                );
                                match &v1056 {
                                    Documents::US26::US26_0(v1056_0_0) => {
                                        let v1062: std::sync::Arc<
                                            std::sync::Mutex<Option<std::process::ChildStdin>>,
                                        > = v387.clone();
                                        let v1064: Result<
                                            std::sync::MutexGuard<Option<std::process::ChildStdin>>,
                                            std::sync::PoisonError<
                                                std::sync::MutexGuard<
                                                    Option<std::process::ChildStdin>,
                                                >,
                                            >,
                                        > = v1062.lock();
                                        let v1079: std::sync::MutexGuard<
                                            Option<std::process::ChildStdin>,
                                        > = Documents::method126(v1064.unwrap());
                                        let v1081: bool = true;
                                        let mut v1079 = v1079;
                                        let v1083: &mut Option<std::process::ChildStdin> =
                                            &mut v1079;
                                        let v1085: Option<std::process::ChildStdin> =
                                            Option::take(v1083);
                                        let v1087: bool = true;
                                        let _optionm_map_ = v1085.map(|x| {
                                            //;
                                            let v1089: std::process::ChildStdin = x;
                                            let v1091: std::sync::Mutex<std::process::ChildStdin> =
                                                std::sync::Mutex::new(v1089);
                                            let v1093: std::sync::Arc<
                                                std::sync::Mutex<std::process::ChildStdin>,
                                            > = std::sync::Arc::new(v1091);
                                            let v1095: bool = true;
                                            v1093
                                        });
                                        let v1097: Option<
                                            std::sync::Arc<
                                                std::sync::Mutex<std::process::ChildStdin>,
                                            >,
                                        > = _optionm_map_;
                                        let v1111: Documents::US27 = defaultValue(
                                            Documents::US27::US27_1,
                                            map(Documents::method127(), v1097),
                                        );
                                        match &v1111 {
                                            Documents::US27::US27_0(v1111_0_0) => {
                                                let v1115: std::sync::Arc<
                                                    std::sync::Mutex<std::process::ChildStdin>,
                                                > = match &v1111 {
                                                    Documents::US27::US27_0(x) => x.clone(),
                                                    _ => unreachable!(),
                                                }
                                                .clone();
                                                (match &v1056 {
                                                    Documents::US26::US26_0(x) => x.clone(),
                                                    _ => unreachable!(),
                                                })(
                                                    v1115.clone()
                                                );
                                                {
                                                    let v1117: std::sync::Arc<
                                                        std::sync::Mutex<std::process::ChildStdin>,
                                                    > = v1115;
                                                    let v1119: Result<
                                                        std::sync::MutexGuard<
                                                            std::process::ChildStdin,
                                                        >,
                                                        std::sync::PoisonError<
                                                            std::sync::MutexGuard<
                                                                std::process::ChildStdin,
                                                            >,
                                                        >,
                                                    > = v1117.lock();
                                                    let v1134: std::sync::MutexGuard<
                                                        std::process::ChildStdin,
                                                    > = Documents::method128(v1119.unwrap());
                                                    let v1136: bool = true;
                                                    let mut v1134 = v1134;
                                                    let v1138: bool = true;
                                                    std::io::Write::flush(&mut *v1134).unwrap();
                                                    ()
                                                }
                                            }
                                            _ => (),
                                        }
                                    }
                                    _ => (),
                                }
                                {
                                    let v1140: std::sync::Arc<
                                        std::sync::Mutex<Option<std::process::Child>>,
                                    > = v261;
                                    let v1142: Result<
                                        std::sync::MutexGuard<Option<std::process::Child>>,
                                        std::sync::PoisonError<
                                            std::sync::MutexGuard<Option<std::process::Child>>,
                                        >,
                                    > = v1140.lock();
                                    let v1157: std::sync::MutexGuard<Option<std::process::Child>> =
                                        Documents::method111(v1142.unwrap());
                                    let v1159: bool = true;
                                    let mut v1157 = v1157;
                                    let v1161: &mut Option<std::process::Child> = &mut v1157;
                                    let v1163: Option<std::process::Child> = Option::take(v1161);
                                    let v1165: std::process::Child = v1163.unwrap();
                                    let v1167: Result<std::process::Output, std::io::Error> =
                                        v1165.wait_with_output();
                                    let v1168 = Documents::method65();
                                    let v1180: Result<std::process::Output, std::string::String> =
                                        v1167.map_err(|x| v1168(x));
                                    let v1183: std::thread::JoinHandle<Result<(), string>> =
                                        Documents::method129(v1042);
                                    let v1186: string = string("v1183");
                                    let v1187: std::thread::JoinHandle<Result<(), string>> =
                                        Documents::method129(v702);
                                    let v1192: string = append(
                                        (append(
                                            string("vec!["),
                                            (append(string("v1187, "), (v1186))),
                                        )),
                                        string("]"),
                                    );
                                    let v1193: Vec<std::thread::JoinHandle<Result<(), string>>> =
                                        vec![v1187, v1183];
                                    let v1195: bool = true;
                                    v1193.into_iter().for_each(|x| {
                                        //;
                                        let v1197: std::thread::JoinHandle<Result<(), string>> = x;
                                        let v1199: Result<
                                            Result<(), string>,
                                            Box<dyn core::any::Any + 'static + Send>,
                                        > = std::thread::JoinHandle::join(v1197);
                                        let v1202: Result<(), string> = v1199.unwrap();
                                        v1202.unwrap();
                                        {
                                            let v1219: bool = true;
                                            let v1221: bool = true;
                                        }
                                    });
                                    {
                                        //;
                                        let v1222 = Documents::method130();
                                        let v1223 = Documents::method131();
                                        let v1224: Documents::US28 = match &v1180 {
                                            Err(v1180_1_0) => v1223(v1180_1_0.clone()),
                                            Ok(v1180_0_0) => v1222(v1180_0_0.clone()),
                                        };
                                        match &v1224 {
                                            Documents::US28::US28_0(v1224_0_0) => {
                                                let v1227: std::process::ExitStatus =
                                                    v1224_0_0.clone().status;
                                                let v1229: Option<i32> = v1227.code();
                                                let v1243: Documents::US29 = defaultValue(
                                                    Documents::US29::US29_1,
                                                    map(Documents::method133(), v1229),
                                                );
                                                match &v1243 {
                                                    Documents::US29::US29_0(v1243_0_0) => (
                                                        match &v1243 {
                                                            Documents::US29::US29_0(x) => x.clone(),
                                                            _ => unreachable!(),
                                                        },
                                                        Documents::US6::US6_1,
                                                        Documents::US24::US24_0(v408.clone()),
                                                    ),
                                                    _ => {
                                                        let v1253:
                                                                                        &str =
                                                                                    &*string("runtime.execute_with_options / exit_code=None");
                                                        (
                                                            -1_i32,
                                                            Documents::US6::US6_0(String::from(
                                                                v1253,
                                                            )),
                                                            Documents::US24::US24_0(v408.clone()),
                                                        )
                                                    }
                                                }
                                            }
                                            Documents::US28::US28_1(v1224_1_0) => {
                                                let v1307: std::string::String = v1224_1_0.clone();
                                                let v1310: () = {
                                                    Documents::closure57(v1307.clone(), ());
                                                    ()
                                                };
                                                (
                                                    -2_i32,
                                                    Documents::US6::US6_0(v1307),
                                                    Documents::US24::US24_1,
                                                )
                                            }
                                        }
                                    }
                                }
                            }
                            Documents::US23::US23_1(v260_1_0) => {
                                let v1358: std::string::String = v260_1_0.clone();
                                let v1361: () = {
                                    Documents::closure44(v1358.clone(), ());
                                    ()
                                };
                                (
                                    -1_i32,
                                    Documents::US6::US6_0(v1358),
                                    Documents::US24::US24_1,
                                )
                            }
                        };
                        let v1408: Documents::US24 = patternInput_3.2.clone();
                        let v1407: Documents::US6 = patternInput_3.1.clone();
                        let v1406: i32 = patternInput_3.0.clone();
                        let v1415: Option<
                            std::sync::Arc<
                                std::sync::Mutex<
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                >,
                            >,
                        > = match &v1408 {
                            Documents::US24::US24_0(v1408_0_0) => Some(
                                match &v1408 {
                                    Documents::US24::US24_0(x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone(),
                            ),
                            _ => {
                                None::<
                                    std::sync::Arc<
                                        std::sync::Mutex<
                                            std::sync::Arc<
                                                std::sync::mpsc::Receiver<std::string::String>,
                                            >,
                                        >,
                                    >,
                                >
                            }
                        };
                        let v1417: bool = true;
                        let _optionm_map_ = v1415.map(|x| {
                            //;
                            let v1419: std::sync::Arc<
                                std::sync::Mutex<
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                >,
                            > = x;
                            let v1421: std::sync::Arc<
                                std::sync::Mutex<
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                >,
                            > = v1419;
                            let v1423: Result<
                                std::sync::MutexGuard<
                                    std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                                >,
                                std::sync::PoisonError<
                                    std::sync::MutexGuard<
                                        std::sync::Arc<
                                            std::sync::mpsc::Receiver<std::string::String>,
                                        >,
                                    >,
                                >,
                            > = v1421.lock();
                            let v1426: std::sync::MutexGuard<
                                std::sync::Arc<std::sync::mpsc::Receiver<std::string::String>>,
                            > = v1423.unwrap();
                            let v1439 = v1426.iter();
                            let v1441: Vec<std::string::String> = v1439.collect::<Vec<_>>();
                            let v1443: bool = true;
                            let _vec_map: Vec<_> = v1441
                                .into_iter()
                                .map(|x| {
                                    //;
                                    let v1445: std::string::String = x;
                                    let v1447: string =
                                        fable_library_rust::String_::fromString(v1445);
                                    let v1449: bool = true;
                                    v1447
                                })
                                .collect::<Vec<_>>();
                            let v1451: Vec<string> = _vec_map;
                            let v1454: LrcPtr<dyn IEnumerable_1<string>> =
                                ofArray_1(fable_library_rust::NativeArray_::array_from(v1451));
                            let v1460: string = join(Documents::method134(), toArray_1(v1454));
                            let v1465: bool = true;
                            v1460
                        });
                        let v1467: Option<string> = _optionm_map_;
                        let v1474: Documents::US5 = match &v1407 {
                            Documents::US6::US6_0(v1407_0_0) => {
                                Documents::US5::US5_0(fable_library_rust::String_::fromString(
                                    match &v1407 {
                                        Documents::US6::US6_0(x) => x.clone(),
                                        _ => unreachable!(),
                                    }
                                    .clone(),
                                ))
                            }
                            _ => Documents::US5::US5_1,
                        };
                        let v1479: string = defaultValue(
                            match &v1474 {
                                Documents::US5::US5_0(v1474_0_0) => match &v1474 {
                                    Documents::US5::US5_0(x) => x.clone(),
                                    _ => unreachable!(),
                                }
                                .clone(),
                                _ => string(""),
                            },
                            v1467,
                        );
                        let v1484: () = {
                            Documents::closure59(v1406, v1479.clone(), ());
                            ()
                        };
                        LrcPtr::new((v1406, v1479))
                    }
                }))();
                _v138.set(Some(x_3))
            }
            {
                let v1526: LrcPtr<(i32, string)> = match &_v138.get().clone() {
                    None => panic!("{}", string("base.capture / _v138=None"),),
                    Some(_v138_0_0) => _v138_0_0.clone(),
                };
                let _v7: (i32, string) = (v1526.0.clone(), v1526.1.clone());
                (_v7.0.clone(), _v7.1.clone())
            }
        }
        pub fn method156(v0_1: string, v1_1: string, v2: string) -> (string, string) {
            let v4: string = Documents::method26(v2, Documents::method45(v1_1.clone()));
            let v8: string = getSlice(
                v1_1.clone(),
                Some(0_i32),
                Some((lastIndexOf(v1_1.clone(), string("."))) - 1_i32),
            );
            let v14: string = getSlice(
                v4.clone(),
                Some(0_i32),
                Some((lastIndexOf(v4.clone(), string("."))) - 1_i32),
            );
            let v22: bool = (endsWith3(v0_1.clone(), string(".md"), false)) == false;
            (
                if v22 {
                    concat(new_array(&[v1_1, string("."), v0_1.clone()]))
                } else {
                    concat(new_array(&[v8, string("."), v0_1.clone()]))
                },
                if v22 {
                    concat(new_array(&[v4, string("."), v0_1.clone()]))
                } else {
                    concat(new_array(&[v14, string("."), v0_1]))
                },
            )
        }
        pub fn method157() -> string {
            string("")
        }
        pub fn closure69(unitVar: (), v0_1: string) -> Documents::US33 {
            Documents::US33::US33_0(v0_1)
        }
        pub fn method158() -> Func1<string, Documents::US33> {
            Func1::new(move |v: string| Documents::closure69((), v))
        }
        pub fn closure70(unitVar: (), v0_1: std::string::String) -> Documents::US33 {
            Documents::US33::US33_1(v0_1)
        }
        pub fn method159() -> Func1<std::string::String, Documents::US33> {
            Func1::new(move |v: std::string::String| Documents::closure70((), v))
        }
        pub fn method160(v0_1: string) -> bool {
            let v4: &str = &*v0_1;
            let v28: std::string::String = String::from(v4);
            let v69: std::path::PathBuf = std::path::PathBuf::from(v28);
            if v69.clone().exists() {
                v69.is_file()
            } else {
                false
            }
        }
        pub fn method162(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: i32,
            v4: string,
            v5: i32,
            v6: string,
            v7: string,
            v8: Documents::US5,
            v9: string,
            v10: string,
        ) -> string {
            let v12: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v19: () = {
                Documents::closure8(v12.clone(), string("{ "), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v12.clone(), string("file"), ());
                ()
            };
            let v37: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v45: () = {
                Documents::closure8(v12.clone(), v0_1, ());
                ()
            };
            let v54: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v63: () = {
                Documents::closure8(v12.clone(), string("real_path"), ());
                ()
            };
            let v71: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v79: () = {
                Documents::closure8(v12.clone(), v1_1, ());
                ()
            };
            let v87: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v96: () = {
                Documents::closure8(v12.clone(), string("relative_path"), ());
                ()
            };
            let v104: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v112: () = {
                Documents::closure8(v12.clone(), v2, ());
                ()
            };
            let v120: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v129: () = {
                Documents::closure8(v12.clone(), string("origin_hash_exit_code"), ());
                ()
            };
            let v137: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v145: () = {
                Documents::closure8(v12.clone(), sprintf!("{}", v3), ());
                ()
            };
            let v153: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v162: () = {
                Documents::closure8(v12.clone(), string("origin_hash"), ());
                ()
            };
            let v170: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v178: () = {
                Documents::closure8(v12.clone(), v4, ());
                ()
            };
            let v186: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v195: () = {
                Documents::closure8(v12.clone(), string("local_git_hash_exit_code"), ());
                ()
            };
            let v203: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v211: () = {
                Documents::closure8(v12.clone(), sprintf!("{}", v5), ());
                ()
            };
            let v219: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v228: () = {
                Documents::closure8(v12.clone(), string("local_git_hash"), ());
                ()
            };
            let v236: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v244: () = {
                Documents::closure8(v12.clone(), v6, ());
                ()
            };
            let v252: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v261: () = {
                Documents::closure8(v12.clone(), string("hash1"), ());
                ()
            };
            let v269: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v277: () = {
                Documents::closure8(v12.clone(), v7, ());
                ()
            };
            let v285: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v294: () = {
                Documents::closure8(v12.clone(), string("hash2"), ());
                ()
            };
            let v302: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v313: () = {
                Documents::closure8(v12.clone(), sprintf!("{:?}", v8), ());
                ()
            };
            let v321: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v330: () = {
                Documents::closure8(v12.clone(), string("dist_path"), ());
                ()
            };
            let v338: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v346: () = {
                Documents::closure8(v12.clone(), v9, ());
                ()
            };
            let v354: () = {
                Documents::closure8(v12.clone(), string("; "), ());
                ()
            };
            let v363: () = {
                Documents::closure8(v12.clone(), string("cache_path"), ());
                ()
            };
            let v371: () = {
                Documents::closure8(v12.clone(), string(" = "), ());
                ()
            };
            let v379: () = {
                Documents::closure8(v12.clone(), v10, ());
                ()
            };
            let v388: () = {
                Documents::closure8(v12.clone(), string(" }"), ());
                ()
            };
            v12.l0.get().clone()
        }
        pub fn method161(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
            v10: string,
            v11: string,
            v12: i32,
            v13: string,
            v14: i32,
            v15: string,
            v16: string,
            v17: Documents::US5,
            v18: string,
            v19: string,
        ) -> string {
            let v20: string =
                Documents::method162(v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                v8,
                v20
            ))
        }
        pub fn closure71(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: string,
            v4: i32,
            v5: string,
            v6: string,
            v7: i32,
            v8: string,
            v9: string,
            v10: Documents::US5,
            unitVar: (),
        ) {
            if Documents::method7(Documents::US0::US0_2) {
                let v15: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v34: Option<i64> = patternInput.5.clone();
                let v33: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v32: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v31: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v30: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v29: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                let v53: string =
                    string("documents.run / par_map / origin_hash |> sm\'.contains local_git_hash |> not / Some hash2 when hash1 = hash2");
                Documents::method19(if (v53.clone()) == string("") {
                    string("")
                } else {
                    Documents::method161(
                        v29.clone(),
                        v30.clone(),
                        v31.clone(),
                        v32.clone(),
                        v33.clone(),
                        v34.clone(),
                        Documents::method8(v29, v30, v31, v32, v33, v34),
                        Documents::method12(),
                        v53,
                        v1_1,
                        v2,
                        v0_1,
                        v4,
                        v3,
                        v7,
                        v6,
                        v9,
                        v10,
                        v5,
                        v8,
                    )
                })
            };
        }
        pub fn closure72(unitVar: (), v0_1: u64) -> Documents::US34 {
            Documents::US34::US34_0(v0_1)
        }
        pub fn method164() -> Func1<u64, Documents::US34> {
            Func1::new(move |v: u64| Documents::closure72((), v))
        }
        pub fn closure73(unitVar: (), v0_1: std::string::String) -> Documents::US34 {
            Documents::US34::US34_1(v0_1)
        }
        pub fn method165() -> Func1<std::string::String, Documents::US34> {
            Func1::new(move |v: std::string::String| Documents::closure73((), v))
        }
        pub fn method167(v0_1: string, v1_1: string, v2: std::string::String) -> string {
            let v4: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v11: () = {
                Documents::closure8(v4.clone(), string("{ "), ());
                ()
            };
            let v20: () = {
                Documents::closure8(v4.clone(), string("old_path"), ());
                ()
            };
            let v29: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v37: () = {
                Documents::closure8(v4.clone(), v0_1, ());
                ()
            };
            let v46: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v55: () = {
                Documents::closure8(v4.clone(), string("new_path"), ());
                ()
            };
            let v63: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v71: () = {
                Documents::closure8(v4.clone(), v1_1, ());
                ()
            };
            let v79: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v88: () = {
                Documents::closure8(v4.clone(), string("error"), ());
                ()
            };
            let v96: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v101: std::string::String = format!("{:#?}", v2);
            let v134: () = {
                Documents::closure8(
                    v4.clone(),
                    fable_library_rust::String_::fromString(v101),
                    (),
                );
                ()
            };
            let v143: () = {
                Documents::closure8(v4.clone(), string(" }"), ());
                ()
            };
            v4.l0.get().clone()
        }
        pub fn method166(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
            v10: std::string::String,
        ) -> string {
            let v11: string = Documents::method167(v8, v9, v10);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("file_system.file_copy"),
                v11
            ))
        }
        pub fn closure74(v0_1: string, v1_1: string, v2: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_3) {
                let v7: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v26: Option<i64> = patternInput.5.clone();
                let v25: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v24: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v23: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v22: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v21: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method166(
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    v26.clone(),
                    Documents::method8(v21, v22, v23, v24, v25, v26),
                    Documents::method31(),
                    v1_1,
                    v0_1,
                    v2,
                ))
            };
        }
        pub fn method169(v0_1: string, v1_1: string, v2: u64) -> string {
            let v4: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v11: () = {
                Documents::closure8(v4.clone(), string("{ "), ());
                ()
            };
            let v20: () = {
                Documents::closure8(v4.clone(), string("old_path"), ());
                ()
            };
            let v29: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v37: () = {
                Documents::closure8(v4.clone(), v0_1, ());
                ()
            };
            let v46: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v55: () = {
                Documents::closure8(v4.clone(), string("new_path"), ());
                ()
            };
            let v63: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v71: () = {
                Documents::closure8(v4.clone(), v1_1, ());
                ()
            };
            let v79: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v88: () = {
                Documents::closure8(v4.clone(), string("result"), ());
                ()
            };
            let v96: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v104: () = {
                Documents::closure8(v4.clone(), sprintf!("{}", v2), ());
                ()
            };
            let v113: () = {
                Documents::closure8(v4.clone(), string(" }"), ());
                ()
            };
            v4.l0.get().clone()
        }
        pub fn method168(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
            v10: u64,
        ) -> string {
            let v11: string = Documents::method169(v8, v9, v10);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("file_system.file_copy"),
                v11
            ))
        }
        pub fn closure75(v0_1: string, v1_1: string, v2: u64, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_1) {
                let v7: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v26: Option<i64> = patternInput.5.clone();
                let v25: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v24: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v23: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v22: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v21: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method168(
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    v26.clone(),
                    Documents::method8(v21, v22, v23, v24, v25, v26),
                    Documents::method62(),
                    v1_1,
                    v0_1,
                    v2,
                ))
            };
        }
        pub fn method163(v0_1: string, v1_1: string) {
            let v4: Result<u64, std::io::Error> = std::fs::copy(&*v1_1.clone(), &*v0_1.clone());
            let v5 = Documents::method65();
            let v17: Result<u64, std::string::String> = v4.map_err(|x| v5(x));
            let v20 = Documents::method164();
            let v21 = Documents::method165();
            let v22: Documents::US34 = match &v17 {
                Err(v17_1_0) => v21(v17_1_0.clone()),
                Ok(v17_0_0) => v20(v17_0_0.clone()),
            };
            match &v22 {
                Documents::US34::US34_0(v22_0_0) => {
                    let v26: () = {
                        Documents::closure75(v0_1.clone(), v1_1.clone(), v22_0_0.clone(), ());
                        ()
                    };
                    ()
                }
                Documents::US34::US34_1(v22_1_0) => {
                    let v69: () = {
                        Documents::closure74(v0_1.clone(), v1_1.clone(), v22_1_0.clone(), ());
                        ()
                    };
                    ()
                }
            }
            ()
        }
        pub fn method171(v0_1: Vec<u8>) -> Vec<u8> {
            v0_1
        }
        pub fn method172(v0_1: i32, v1_1: LrcPtr<Documents::Mut7>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method173(v0_1: string) -> string {
            v0_1
        }
        pub fn method174(
            v0_1: std::sync::MutexGuard<std::process::ChildStdin>,
        ) -> std::sync::MutexGuard<std::process::ChildStdin> {
            v0_1
        }
        pub fn closure78(
            v0_1: string,
            v1_1: std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>,
        ) {
            let v3: std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>> = v1_1;
            let v5: Result<
                std::sync::MutexGuard<std::process::ChildStdin>,
                std::sync::PoisonError<std::sync::MutexGuard<std::process::ChildStdin>>,
            > = v3.lock();
            let v8: std::sync::MutexGuard<std::process::ChildStdin> = v5.unwrap();
            let v20: string = Documents::method173(v0_1);
            let v22: &[u8] = v20.as_bytes();
            let v23: std::sync::MutexGuard<std::process::ChildStdin> = Documents::method174(v8);
            let v25: bool = true;
            let mut v23 = v23;
            let v27: bool = true;
            std::io::Write::write_all(&mut *v23, v22).unwrap();
            ()
        }
        pub fn method175(v0_1: i32, v1_1: LrcPtr<Documents::Mut8>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method177(v0_1: i32, v1_1: i32, v2: string) -> string {
            let v4: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v11: () = {
                Documents::closure8(v4.clone(), string("{ "), ());
                ()
            };
            let v20: () = {
                Documents::closure8(v4.clone(), string("exit_code"), ());
                ()
            };
            let v29: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v37: () = {
                Documents::closure8(v4.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v46: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v55: () = {
                Documents::closure8(v4.clone(), string("result_len"), ());
                ()
            };
            let v63: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v71: () = {
                Documents::closure8(v4.clone(), sprintf!("{}", v1_1), ());
                ()
            };
            let v79: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v88: () = {
                Documents::closure8(v4.clone(), string("output_path"), ());
                ()
            };
            let v96: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v104: () = {
                Documents::closure8(v4.clone(), v2, ());
                ()
            };
            let v113: () = {
                Documents::closure8(v4.clone(), string(" }"), ());
                ()
            };
            v4.l0.get().clone()
        }
        pub fn method176(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: i32,
            v10: string,
        ) -> string {
            let v11: string = Documents::method177(v8, v9, v10);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.hangul"),
                v11
            ))
        }
        pub fn closure79(v0_1: string, v1_1: i32, v2: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_2) {
                let v7: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v26: Option<i64> = patternInput.5.clone();
                let v25: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v24: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v23: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v22: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v21: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method176(
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    v26.clone(),
                    Documents::method8(v21, v22, v23, v24, v25, v26),
                    Documents::method12(),
                    v1_1,
                    length(v2),
                    v0_1,
                ))
            };
        }
        pub fn method170(v0_1: string, v1_1: string, v2: string, v3: string) -> Documents::US35 {
            let v7: Result<Vec<u8>, std::io::Error> = std::fs::read(&*v3);
            let v45: Vec<u8> = Documents::method171(v7.unwrap());
            let v47: Result<std::string::String, std::string::FromUtf8Error> =
                std::string::String::from_utf8(v45);
            let v50: std::string::String = v47.unwrap();
            let v86: Array<string> = split(
                fable_library_rust::String_::fromString(v50),
                string("\n"),
                -1_i32,
                0_i32,
            );
            let v89: i32 = get_Count(v86.clone());
            let v90: Array<string> = new_init(&string(""), v89);
            let v91: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                l0: MutCell::new(0_i32),
            });
            while Documents::method59(v89, v91.clone()) {
                let v93: i32 = v91.l0.get().clone();
                let v96: string = trim(v86[v93].clone());
                v90.get_mut()[v93 as usize] = v96;
                {
                    let v99: i32 = (v93) + 1_i32;
                    v91.l0.set(v99);
                    ()
                }
            }
            {
                let v100: i32 = get_Count(v90.clone());
                let v101: Array<string> = new_init(&string(""), v100);
                let v102: LrcPtr<Documents::Mut7> = LrcPtr::new(Documents::Mut7 {
                    l0: MutCell::new(0_i32),
                    l1: MutCell::new(0_i32),
                });
                while Documents::method172(v100, v102.clone()) {
                    let v104: i32 = v102.l0.get().clone();
                    let v105: i32 = v102.l1.get().clone();
                    let v106: string = v90[v104].clone();
                    let v112: i32 = if string("") != (v106.clone()) {
                        v101.get_mut()[v105 as usize] = v106;
                        (v105) + 1_i32
                    } else {
                        v105
                    };
                    let v113: i32 = (v104) + 1_i32;
                    v102.l0.set(v113);
                    v102.l1.set(v112);
                    ()
                }
                {
                    let v114: i32 = v102.l1.get().clone();
                    let v115: Array<string> = new_init(&string(""), v114);
                    let v116: LrcPtr<Documents::Mut6> = LrcPtr::new(Documents::Mut6 {
                        l0: MutCell::new(0_i32),
                    });
                    while Documents::method59(v114, v116.clone()) {
                        let v118: i32 = v116.l0.get().clone();
                        let v119: string = v101[v118].clone();
                        v115.get_mut()[v118 as usize] = v119;
                        {
                            let v120: i32 = (v118) + 1_i32;
                            v116.l0.set(v120);
                            ()
                        }
                    }
                    {
                        let _v121: LrcPtr<dyn IEnumerable_1<string>> = delay(Func0::new({
                            let v115 = v115.clone();
                            move || {
                                map_1(
                                    Func1::new({
                                        let v115 = v115.clone();
                                        move |i: i32| v115[i].clone()
                                    }),
                                    rangeNumeric(0_i32, 1_i32, (get_Count(v115.clone())) - 1_i32),
                                )
                            }
                        }));
                        let v136: string = concat(new_array(&[
                            join(Documents::method134(), toArray_1(_v121)),
                            string("\n\n"),
                        ]));
                        let patternInput: (i32, string) = Documents::method81(
                            concat(new_array(&[
                                Documents::method26(
                                    v0_1,
                                    concat(new_array(&[
                                        string("../vault/deps/hangulize/cmd/hangulize/hangulize"),
                                        if cfg!(windows) {
                                            string(".exe")
                                        } else {
                                            string("")
                                        },
                                    ])),
                                ),
                                string(" "),
                                v1_1,
                            ])),
                            None::<CancellationToken>,
                            new_empty::<(string, string)>(),
                            None::<Func1<(i32, string, bool), Arc<Async<()>>>>,
                            Some(Func1::new({
                                let v136 = v136.clone();
                                move |v: std::sync::Arc<
                                    std::sync::Mutex<std::process::ChildStdin>,
                                >| {
                                    Documents::closure78(v136.clone(), v)
                                }
                            })),
                            true,
                            None::<string>,
                        );
                        let v180: i32 = patternInput.0.clone();
                        let v182: Array<string> =
                            split(patternInput.1.clone(), string("\n"), -1_i32, 0_i32);
                        let v185: i32 = get_Count(v182.clone());
                        let v187: LrcPtr<Documents::Mut8> = LrcPtr::new(Documents::Mut8 {
                            l0: MutCell::new(0_i32),
                            l1: MutCell::new(string("")),
                            l2: MutCell::new(0_i32),
                            l3: MutCell::new(0_i32),
                        });
                        while Documents::method175(v100, v187.clone()) {
                            let v189: i32 = v187.l0.get().clone();
                            let matchValue: string = v187.l1.get().clone();
                            let matchValue_1: i32 = v187.l2.get().clone();
                            let v192: i32 = v187.l3.get().clone();
                            let v191: i32 = matchValue_1;
                            let v190: string = matchValue;
                            let patternInput_2: (string, i32, i32) =
                                if (v90[v189].clone()) == string("") {
                                    (
                                        concat(new_array(&[v190.clone(), string("\n")])),
                                        (v191) + 1_i32,
                                        (v192) + 1_i32,
                                    )
                                } else {
                                    let v198: i32 = (v191) - (v192);
                                    (
                                        if (v198) >= (v185) {
                                            v190.clone()
                                        } else {
                                            let v200: string = v182[v198].clone();
                                            if (v198) == ((v185) - 1_i32) {
                                                concat(new_array(&[v190.clone(), v200.clone()]))
                                            } else {
                                                concat(new_array(&[v190, v200, string("\n")]))
                                            }
                                        },
                                        (v191) + 1_i32,
                                        v192,
                                    )
                                };
                            let v213: i32 = (v189) + 1_i32;
                            v187.l0.set(v213);
                            v187.l1.set(patternInput_2.0.clone());
                            v187.l2.set(patternInput_2.1.clone());
                            v187.l3.set(patternInput_2.2.clone());
                            ()
                        }
                        {
                            let matchValue_3: string = v187.l1.get().clone();
                            let matchValue_4: i32 = v187.l2.get().clone();
                            let matchValue_5: i32 = v187.l3.get().clone();
                            let v214: string = matchValue_3;
                            std::fs::write(&*v2.clone(), &*v214.clone()).unwrap();
                            {
                                let v221: () = {
                                    Documents::closure79(v2, v180, v214.clone(), ());
                                    ()
                                };
                                Documents::US35::US35_0(v180, v214)
                            }
                        }
                    }
                }
            }
        }
        pub fn method179(v0_1: i32, v1_1: string) -> string {
            let v3: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v10: () = {
                Documents::closure8(v3.clone(), string("{ "), ());
                ()
            };
            let v19: () = {
                Documents::closure8(v3.clone(), string("exit_code"), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v36: () = {
                Documents::closure8(v3.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v45: () = {
                Documents::closure8(v3.clone(), string("; "), ());
                ()
            };
            let v54: () = {
                Documents::closure8(v3.clone(), string("result"), ());
                ()
            };
            let v62: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v70: () = {
                Documents::closure8(v3.clone(), v1_1, ());
                ()
            };
            let v79: () = {
                Documents::closure8(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method178(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: string,
        ) -> string {
            let v10: string = Documents::method179(v8, v9);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.files_fn / error"),
                v10
            ))
        }
        pub fn closure80(v0_1: string, v1_1: i32, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_2) {
                let v6: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v25: Option<i64> = patternInput.5.clone();
                let v24: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v23: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v22: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v21: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v20: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method178(
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    Documents::method8(v20, v21, v22, v23, v24, v25),
                    Documents::method12(),
                    v1_1,
                    v0_1,
                ))
            };
        }
        pub fn closure77(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: string,
            v4: string,
            v5: string,
        ) -> Documents::US32 {
            let patternInput: (string, string) = Documents::method156(v5, v4.clone(), v0_1);
            let v7: string = patternInput.1.clone();
            let v6: string = patternInput.0.clone();
            if if if (Documents::method160(v6.clone())) == false {
                true
            } else {
                (Documents::method160(v7.clone())) == false
            } {
                false
            } else {
                let v13: string = Documents::method36(v6.clone());
                let v15: Result<std::fs::File, std::io::Error> = std::fs::File::open(&*v13);
                let v18: std::fs::File = v15.unwrap();
                let v31: std::io::BufReader<std::fs::File> = std::io::BufReader::new(v18);
                let v33: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                    std::io::BufReader::new(v31);
                let v35: bool = true;
                let mut v33 = v33;
                let result: sha2::Sha256 = sha2::Digest::new();
                {
                    let v38: sha2::Sha256 = result;
                    let v40: bool = true;
                    let mut v38 = v38;
                    let v41: usize = 0_i32 as usize;
                    let v45: _ = [0_u8; 1024];
                    let v47: bool = true;
                    loop {
                        // rust.loop;
                        let v49: bool = true;
                        let mut v45 = v45;
                        let v51: Result<usize, std::io::Error> =
                            std::io::Read::read(&mut v33, &mut v45);
                        let v63: usize = v51.unwrap();
                        if (v63) == (v41) {
                            let v70: bool = true;
                            break;
                            ()
                        }
                        {
                            let v71: usize = v63;
                            let v86: &_ = if (v71) == (v45.len()) {
                                &v45[v41..]
                            } else {
                                &v45[v41..v71]
                            };
                            sha2::Digest::update(&mut v38, v86);
                            {
                                let v89: bool = true;
                            } // rust.loop;
                            let v91: bool = true;
                        } // rust.loop;
                        let v93: bool = true;
                    } // rust.loop;
                    let v95: bool = true;
                    {
                        // rust.loop;
                        let v97: bool = true;
                        {
                            // rust.loop;
                            let v99: &[u8] = &sha2::Digest::finalize(v38);
                            let v101: Vec<u8> = v99.iter().map(|x| *x).collect::<Vec<_>>();
                            let v103: bool = true;
                            let _vec_map: Vec<_> = v101
                                .into_iter()
                                .map(|x| {
                                    //;
                                    let v105: u8 = x;
                                    let v107: std::string::String = format!("{:02x}", v105);
                                    let v109: string =
                                        fable_library_rust::String_::fromString(v107);
                                    let v111: bool = true;
                                    v109
                                })
                                .collect::<Vec<_>>();
                            let v113: Vec<string> = _vec_map;
                            let v115: Array<string> =
                                fable_library_rust::NativeArray_::array_from(v113);
                            let _v116: LrcPtr<dyn IEnumerable_1<string>> = delay(Func0::new({
                                let v115 = v115.clone();
                                move || {
                                    map_1(
                                        Func1::new({
                                            let v115 = v115.clone();
                                            move |i: i32| v115[i].clone()
                                        }),
                                        rangeNumeric(
                                            0_i32,
                                            1_i32,
                                            (get_Count(v115.clone())) - 1_i32,
                                        ),
                                    )
                                }
                            }));
                            let v125: string = Documents::method157();
                            let v131: string = join(
                                if (v125.clone()) == string("\n") {
                                    Documents::method61(v125.clone())
                                } else {
                                    v125
                                },
                                toArray_1(_v116),
                            );
                            let v135 = Documents::method65();
                            let v138: Result<string, std::string::String> =
                                Ok::<string, std::io::Error>(v131).map_err(|x| v135(x));
                            let v150 = Documents::method158();
                            let v151 = Documents::method159();
                            let v153: Documents::US33 = match v138 {
                                Ok(x) => v150(x),
                                Err(e) => v151(e),
                            };
                            let v161: string = match &v153 {
                                Documents::US33::US33_0(v153_0_0) => v153_0_0.clone(),
                                Documents::US33::US33_1(v153_1_0) => panic!(
                                    "{}",
                                    sprintf!(
                                        "resultm.get / Result value was Error: {}",
                                        v153_1_0.clone()
                                    ),
                                ),
                            };
                            let v162: string = Documents::method36(v7.clone());
                            let v164: Result<std::fs::File, std::io::Error> =
                                std::fs::File::open(&*v162);
                            let v167: std::fs::File = v164.unwrap();
                            let v180: std::io::BufReader<std::fs::File> =
                                std::io::BufReader::new(v167);
                            let v182: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                                std::io::BufReader::new(v180);
                            let v184: bool = true;
                            let mut v182 = v182;
                            let result: sha2::Sha256 = sha2::Digest::new();
                            {
                                let v187: sha2::Sha256 = result;
                                let v189: bool = true;
                                let mut v187 = v187;
                                let v190: usize = 0_i32 as usize;
                                let v194: _ = [0_u8; 1024];
                                let v196: bool = true;
                                loop {
                                    // rust.loop;
                                    let v198: bool = true;
                                    let mut v194 = v194;
                                    let v200: Result<usize, std::io::Error> =
                                        std::io::Read::read(&mut v182, &mut v194);
                                    let v212: usize = v200.unwrap();
                                    if (v212) == (v190) {
                                        let v219: bool = true;
                                        break;
                                        ()
                                    }
                                    {
                                        let v220: usize = v212;
                                        let v235: &_ = if (v220) == (v194.len()) {
                                            &v194[v190..]
                                        } else {
                                            &v194[v190..v220]
                                        };
                                        sha2::Digest::update(&mut v187, v235);
                                        {
                                            let v238: bool = true;
                                        } // rust.loop;
                                        let v240: bool = true;
                                    } // rust.loop;
                                    let v242: bool = true;
                                } // rust.loop;
                                let v244: bool = true;
                                {
                                    // rust.loop;
                                    let v246: bool = true;
                                    {
                                        // rust.loop;
                                        let v248: &[u8] = &sha2::Digest::finalize(v187);
                                        let v250: Vec<u8> =
                                            v248.iter().map(|x| *x).collect::<Vec<_>>();
                                        let v252: bool = true;
                                        let _vec_map: Vec<_> = v250
                                            .into_iter()
                                            .map(|x| {
                                                //;
                                                let v254: u8 = x;
                                                let v256: std::string::String =
                                                    format!("{:02x}", v254);
                                                let v258: string =
                                                    fable_library_rust::String_::fromString(v256);
                                                let v260: bool = true;
                                                v258
                                            })
                                            .collect::<Vec<_>>();
                                        let v262: Vec<string> = _vec_map;
                                        let v264: Array<string> =
                                            fable_library_rust::NativeArray_::array_from(v262);
                                        let _v265: LrcPtr<dyn IEnumerable_1<string>> =
                                            delay(Func0::new({
                                                let v264 = v264.clone();
                                                move || {
                                                    map_1(
                                                        Func1::new({
                                                            let v264 = v264.clone();
                                                            move |i_1: i32| v264[i_1].clone()
                                                        }),
                                                        rangeNumeric(
                                                            0_i32,
                                                            1_i32,
                                                            (get_Count(v264.clone())) - 1_i32,
                                                        ),
                                                    )
                                                }
                                            }));
                                        let v274: string = Documents::method157();
                                        let v280: string = join(
                                            if (v274.clone()) == string("\n") {
                                                Documents::method61(v274.clone())
                                            } else {
                                                v274
                                            },
                                            toArray_1(_v265),
                                        );
                                        let v284 = Documents::method65();
                                        let v287: Result<string, std::string::String> =
                                            Ok::<string, std::io::Error>(v280).map_err(|x| v284(x));
                                        let v299 = Documents::method158();
                                        let v300 = Documents::method159();
                                        let v302: Documents::US33 = match v287 {
                                            Ok(x) => v299(x),
                                            Err(e) => v300(e),
                                        };
                                        (v161)
                                            == (match &v302 {
                                                Documents::US33::US33_0(v302_0_0) => {
                                                    v302_0_0.clone()
                                                }
                                                Documents::US33::US33_1(v302_1_0) => panic!(
                                                    "{}",
                                                    sprintf!(
                                                        "resultm.get / Result value was Error: {}",
                                                        v302_1_0.clone()
                                                    ),
                                                ),
                                            })
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                Documents::US32::US32_1
            } else {
                let v314: Documents::US35 = Documents::method170(v2, v3, v6.clone(), v4);
                match &v314 {
                    Documents::US35::US35_0(v314_0_0, v314_0_1) => {
                        let v316: string = v314_0_1.clone();
                        let v315: i32 = v314_0_0.clone();
                        if (v315) != 0_i32 {
                            let v322: () = {
                                Documents::closure80(v316.clone(), v315, ());
                                ()
                            };
                            Documents::US32::US32_0(Err::<string, LrcPtr<(string, string)>>(
                                LrcPtr::new((v6.clone(), v316)),
                            ))
                        } else {
                            if Documents::method160(v6.clone()) {
                                Documents::method163(v7.clone(), v6.clone())
                            } else {
                                panic!(
                                    "{}",
                                    concat(new_array(&[
                                        string("documents.files_fn / "),
                                        v6.clone(),
                                        string(" should exist")
                                    ])),
                                )
                            }
                            Documents::US32::US32_0(Ok::<string, LrcPtr<(string, string)>>(
                                v6.clone(),
                            ))
                        }
                    }
                    Documents::US35::US35_1(v314_1_0, v314_1_1) => {
                        Documents::US32::US32_0(Err::<string, LrcPtr<(string, string)>>(
                            LrcPtr::new((v6.clone(), v314_1_1.clone())),
                        ))
                    }
                }
            }
        }
        pub fn closure76(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: string,
            v4: string,
        ) -> Func1<string, Documents::US32> {
            Func1::new({
                let v0_1 = v0_1.clone();
                let v1_1 = v1_1.clone();
                let v2 = v2.clone();
                let v3 = v3.clone();
                let v4 = v4.clone();
                move |v: string| {
                    Documents::closure77(
                        v0_1.clone(),
                        v1_1.clone(),
                        v2.clone(),
                        v3.clone(),
                        v4.clone(),
                        v,
                    )
                }
            })
        }
        pub fn method182(v0_1: i32, v1_1: string, v2: string) -> string {
            let v4: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v11: () = {
                Documents::closure8(v4.clone(), string("{ "), ());
                ()
            };
            let v20: () = {
                Documents::closure8(v4.clone(), string("exit_code"), ());
                ()
            };
            let v29: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v37: () = {
                Documents::closure8(v4.clone(), sprintf!("{}", v0_1), ());
                ()
            };
            let v46: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v55: () = {
                Documents::closure8(v4.clone(), string("output_path"), ());
                ()
            };
            let v63: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v71: () = {
                Documents::closure8(v4.clone(), v1_1, ());
                ()
            };
            let v79: () = {
                Documents::closure8(v4.clone(), string("; "), ());
                ()
            };
            let v88: () = {
                Documents::closure8(v4.clone(), string("result"), ());
                ()
            };
            let v96: () = {
                Documents::closure8(v4.clone(), string(" = "), ());
                ()
            };
            let v104: () = {
                Documents::closure8(v4.clone(), v2, ());
                ()
            };
            let v113: () = {
                Documents::closure8(v4.clone(), string(" }"), ());
                ()
            };
            v4.l0.get().clone()
        }
        pub fn method181(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: i32,
            v9: string,
            v10: string,
        ) -> string {
            let v11: string = Documents::method182(v8, v9, v10);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.crowbook / result contains ERROR"),
                v11
            ))
        }
        pub fn closure83(v0_1: string, v1_1: string, v2: i32, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_3) {
                let v7: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v26: Option<i64> = patternInput.5.clone();
                let v25: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v24: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v23: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v22: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v21: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method181(
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    v26.clone(),
                    Documents::method8(v21, v22, v23, v24, v25, v26),
                    Documents::method31(),
                    v2,
                    v0_1,
                    v1_1,
                ))
            };
        }
        pub fn method180(
            v0_1: bool,
            v1_1: string,
            v2: string,
            v3: string,
            v4: string,
        ) -> Documents::US35 {
            let v61: string = if string("html") == (v4.clone()) {
                string("--set html.css.add \\\"\' body { color: #e8e6e3; background-color: #202324; } a { color: #989693; } pre { background-color: #1b1b1b; padding: 10px; } \'\\\" rendering.num_depth 6 rendering.highlight.theme \\\"Solarized (dark)\\\"")
            } else {
                if string("pdf") == (v4.clone()) {
                    append((append(string("--set tex.paper.size a4paper tex.template.add \"\\pagenumbering{gobble}\""),
                                       (if (v0_1) == false {
                                            string("")
                                        } else {
                                            string(" tex.template.add \"\\usepackage{polyglossia}\" tex.template.add \"\\setmainlanguage{korean}\" tex.template.add \"\\setmainfont{NanumGothicCoding}\" tex.font.size 13")
                                        }))),
                               string(" rendering.num_depth 6 rendering.highlight.theme \\\"Solarized (dark)\\\""))
                } else {
                    if string("epub") == (v4.clone()) {
                        string("--set epub.version 3 html.css.add \\\"\'  body { color: #e8e6e3; background-color: #202324; }  a { color: #989693; }  \'\\\" rendering.num_depth 6 rendering.highlight.theme \\\"Solarized (dark)\\\"")
                    } else {
                        string("")
                    }
                }
            };
            let patternInput: (i32, string) = Documents::method81(
                append(
                    (concat(new_array(&[string("crowbook --verbose --to "), v4]))),
                    sprintf!(" --single \"{}\" --output \"{}\" {}", v2, v1_1.clone(), v61),
                ),
                None::<CancellationToken>,
                new_empty::<(string, string)>(),
                None::<Func1<(i32, string, bool), Arc<Async<()>>>>,
                None::<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
                true,
                Some(v3),
            );
            let v75: string = patternInput.1.clone();
            let v74: i32 = patternInput.0.clone();
            if (contains(v75.clone(), string("ERROR"))) == false {
                Documents::US35::US35_0(v74, v75.clone())
            } else {
                let v84: () = {
                    Documents::closure83(v1_1, v75.clone(), v74, ());
                    ()
                };
                Documents::US35::US35_1(v74, v75)
            }
        }
        pub fn closure82(
            v0_1: string,
            v1_1: string,
            v2: bool,
            v3: string,
            v4: string,
        ) -> Documents::US32 {
            let patternInput: (string, string) = Documents::method156(v4.clone(), v3.clone(), v0_1);
            let v6: string = patternInput.1.clone();
            let v5: string = patternInput.0.clone();
            if if if (Documents::method160(v5.clone())) == false {
                true
            } else {
                (Documents::method160(v6.clone())) == false
            } {
                false
            } else {
                let v12: string = Documents::method36(v5.clone());
                let v14: Result<std::fs::File, std::io::Error> = std::fs::File::open(&*v12);
                let v17: std::fs::File = v14.unwrap();
                let v30: std::io::BufReader<std::fs::File> = std::io::BufReader::new(v17);
                let v32: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                    std::io::BufReader::new(v30);
                let v34: bool = true;
                let mut v32 = v32;
                let result: sha2::Sha256 = sha2::Digest::new();
                {
                    let v37: sha2::Sha256 = result;
                    let v39: bool = true;
                    let mut v37 = v37;
                    let v40: usize = 0_i32 as usize;
                    let v44: _ = [0_u8; 1024];
                    let v46: bool = true;
                    loop {
                        // rust.loop;
                        let v48: bool = true;
                        let mut v44 = v44;
                        let v50: Result<usize, std::io::Error> =
                            std::io::Read::read(&mut v32, &mut v44);
                        let v62: usize = v50.unwrap();
                        if (v62) == (v40) {
                            let v69: bool = true;
                            break;
                            ()
                        }
                        {
                            let v70: usize = v62;
                            let v85: &_ = if (v70) == (v44.len()) {
                                &v44[v40..]
                            } else {
                                &v44[v40..v70]
                            };
                            sha2::Digest::update(&mut v37, v85);
                            {
                                let v88: bool = true;
                            } // rust.loop;
                            let v90: bool = true;
                        } // rust.loop;
                        let v92: bool = true;
                    } // rust.loop;
                    let v94: bool = true;
                    {
                        // rust.loop;
                        let v96: bool = true;
                        {
                            // rust.loop;
                            let v98: &[u8] = &sha2::Digest::finalize(v37);
                            let v100: Vec<u8> = v98.iter().map(|x| *x).collect::<Vec<_>>();
                            let v102: bool = true;
                            let _vec_map: Vec<_> = v100
                                .into_iter()
                                .map(|x| {
                                    //;
                                    let v104: u8 = x;
                                    let v106: std::string::String = format!("{:02x}", v104);
                                    let v108: string =
                                        fable_library_rust::String_::fromString(v106);
                                    let v110: bool = true;
                                    v108
                                })
                                .collect::<Vec<_>>();
                            let v112: Vec<string> = _vec_map;
                            let v114: Array<string> =
                                fable_library_rust::NativeArray_::array_from(v112);
                            let _v115: LrcPtr<dyn IEnumerable_1<string>> = delay(Func0::new({
                                let v114 = v114.clone();
                                move || {
                                    map_1(
                                        Func1::new({
                                            let v114 = v114.clone();
                                            move |i: i32| v114[i].clone()
                                        }),
                                        rangeNumeric(
                                            0_i32,
                                            1_i32,
                                            (get_Count(v114.clone())) - 1_i32,
                                        ),
                                    )
                                }
                            }));
                            let v124: string = Documents::method157();
                            let v130: string = join(
                                if (v124.clone()) == string("\n") {
                                    Documents::method61(v124.clone())
                                } else {
                                    v124
                                },
                                toArray_1(_v115),
                            );
                            let v134 = Documents::method65();
                            let v137: Result<string, std::string::String> =
                                Ok::<string, std::io::Error>(v130).map_err(|x| v134(x));
                            let v149 = Documents::method158();
                            let v150 = Documents::method159();
                            let v152: Documents::US33 = match v137 {
                                Ok(x) => v149(x),
                                Err(e) => v150(e),
                            };
                            let v160: string = match &v152 {
                                Documents::US33::US33_0(v152_0_0) => v152_0_0.clone(),
                                Documents::US33::US33_1(v152_1_0) => panic!(
                                    "{}",
                                    sprintf!(
                                        "resultm.get / Result value was Error: {}",
                                        v152_1_0.clone()
                                    ),
                                ),
                            };
                            let v161: string = Documents::method36(v6.clone());
                            let v163: Result<std::fs::File, std::io::Error> =
                                std::fs::File::open(&*v161);
                            let v166: std::fs::File = v163.unwrap();
                            let v179: std::io::BufReader<std::fs::File> =
                                std::io::BufReader::new(v166);
                            let v181: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                                std::io::BufReader::new(v179);
                            let v183: bool = true;
                            let mut v181 = v181;
                            let result: sha2::Sha256 = sha2::Digest::new();
                            {
                                let v186: sha2::Sha256 = result;
                                let v188: bool = true;
                                let mut v186 = v186;
                                let v189: usize = 0_i32 as usize;
                                let v193: _ = [0_u8; 1024];
                                let v195: bool = true;
                                loop {
                                    // rust.loop;
                                    let v197: bool = true;
                                    let mut v193 = v193;
                                    let v199: Result<usize, std::io::Error> =
                                        std::io::Read::read(&mut v181, &mut v193);
                                    let v211: usize = v199.unwrap();
                                    if (v211) == (v189) {
                                        let v218: bool = true;
                                        break;
                                        ()
                                    }
                                    {
                                        let v219: usize = v211;
                                        let v234: &_ = if (v219) == (v193.len()) {
                                            &v193[v189..]
                                        } else {
                                            &v193[v189..v219]
                                        };
                                        sha2::Digest::update(&mut v186, v234);
                                        {
                                            let v237: bool = true;
                                        } // rust.loop;
                                        let v239: bool = true;
                                    } // rust.loop;
                                    let v241: bool = true;
                                } // rust.loop;
                                let v243: bool = true;
                                {
                                    // rust.loop;
                                    let v245: bool = true;
                                    {
                                        // rust.loop;
                                        let v247: &[u8] = &sha2::Digest::finalize(v186);
                                        let v249: Vec<u8> =
                                            v247.iter().map(|x| *x).collect::<Vec<_>>();
                                        let v251: bool = true;
                                        let _vec_map: Vec<_> = v249
                                            .into_iter()
                                            .map(|x| {
                                                //;
                                                let v253: u8 = x;
                                                let v255: std::string::String =
                                                    format!("{:02x}", v253);
                                                let v257: string =
                                                    fable_library_rust::String_::fromString(v255);
                                                let v259: bool = true;
                                                v257
                                            })
                                            .collect::<Vec<_>>();
                                        let v261: Vec<string> = _vec_map;
                                        let v263: Array<string> =
                                            fable_library_rust::NativeArray_::array_from(v261);
                                        let _v264: LrcPtr<dyn IEnumerable_1<string>> =
                                            delay(Func0::new({
                                                let v263 = v263.clone();
                                                move || {
                                                    map_1(
                                                        Func1::new({
                                                            let v263 = v263.clone();
                                                            move |i_1: i32| v263[i_1].clone()
                                                        }),
                                                        rangeNumeric(
                                                            0_i32,
                                                            1_i32,
                                                            (get_Count(v263.clone())) - 1_i32,
                                                        ),
                                                    )
                                                }
                                            }));
                                        let v273: string = Documents::method157();
                                        let v279: string = join(
                                            if (v273.clone()) == string("\n") {
                                                Documents::method61(v273.clone())
                                            } else {
                                                v273
                                            },
                                            toArray_1(_v264),
                                        );
                                        let v283 = Documents::method65();
                                        let v286: Result<string, std::string::String> =
                                            Ok::<string, std::io::Error>(v279).map_err(|x| v283(x));
                                        let v298 = Documents::method158();
                                        let v299 = Documents::method159();
                                        let v301: Documents::US33 = match v286 {
                                            Ok(x) => v298(x),
                                            Err(e) => v299(e),
                                        };
                                        (v160)
                                            == (match &v301 {
                                                Documents::US33::US33_0(v301_0_0) => {
                                                    v301_0_0.clone()
                                                }
                                                Documents::US33::US33_1(v301_1_0) => panic!(
                                                    "{}",
                                                    sprintf!(
                                                        "resultm.get / Result value was Error: {}",
                                                        v301_1_0.clone()
                                                    ),
                                                ),
                                            })
                                    }
                                }
                            }
                        }
                    }
                }
            } {
                Documents::US32::US32_1
            } else {
                let v313: Documents::US35 = Documents::method180(v2, v5.clone(), v3, v1_1, v4);
                match &v313 {
                    Documents::US35::US35_0(v313_0_0, v313_0_1) => {
                        let v315: string = v313_0_1.clone();
                        let v314: i32 = v313_0_0.clone();
                        if (v314) != 0_i32 {
                            let v321: () = {
                                Documents::closure80(v315.clone(), v314, ());
                                ()
                            };
                            Documents::US32::US32_0(Err::<string, LrcPtr<(string, string)>>(
                                LrcPtr::new((v5.clone(), v315)),
                            ))
                        } else {
                            if Documents::method160(v5.clone()) {
                                Documents::method163(v6.clone(), v5.clone())
                            } else {
                                panic!(
                                    "{}",
                                    concat(new_array(&[
                                        string("documents.files_fn / "),
                                        v5.clone(),
                                        string(" should exist")
                                    ])),
                                )
                            }
                            Documents::US32::US32_0(Ok::<string, LrcPtr<(string, string)>>(
                                v5.clone(),
                            ))
                        }
                    }
                    Documents::US35::US35_1(v313_1_0, v313_1_1) => {
                        Documents::US32::US32_0(Err::<string, LrcPtr<(string, string)>>(
                            LrcPtr::new((v5.clone(), v313_1_1.clone())),
                        ))
                    }
                }
            }
        }
        pub fn closure81(
            v0_1: string,
            v1_1: string,
            v2: bool,
            v3: string,
        ) -> Func1<string, Documents::US32> {
            Func1::new({
                let v0_1 = v0_1.clone();
                let v1_1 = v1_1.clone();
                let v2 = v2.clone();
                let v3 = v3.clone();
                move |v: string| Documents::closure82(v0_1.clone(), v1_1.clone(), v2, v3.clone(), v)
            })
        }
        pub fn method184(v0_1: string, v1_1: string) -> string {
            let v3: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v10: () = {
                Documents::closure8(v3.clone(), string("{ "), ());
                ()
            };
            let v19: () = {
                Documents::closure8(v3.clone(), string("output_path"), ());
                ()
            };
            let v28: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v36: () = {
                Documents::closure8(v3.clone(), v0_1, ());
                ()
            };
            let v45: () = {
                Documents::closure8(v3.clone(), string("; "), ());
                ()
            };
            let v54: () = {
                Documents::closure8(v3.clone(), string("output_cache_path"), ());
                ()
            };
            let v62: () = {
                Documents::closure8(v3.clone(), string(" = "), ());
                ()
            };
            let v70: () = {
                Documents::closure8(v3.clone(), v1_1, ());
                ()
            };
            let v79: () = {
                Documents::closure8(v3.clone(), string(" }"), ());
                ()
            };
            v3.l0.get().clone()
        }
        pub fn method183(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: string,
            v9: string,
        ) -> string {
            let v10: string = Documents::method184(v8, v9);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.run / par_map / files\' = [] / listm.iter"),
                v10
            ))
        }
        pub fn closure84(v0_1: string, v1_1: string, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_2) {
                let v6: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v25: Option<i64> = patternInput.5.clone();
                let v24: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v23: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v22: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v21: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v20: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method183(
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    v25.clone(),
                    Documents::method8(v20, v21, v22, v23, v24, v25),
                    Documents::method12(),
                    v1_1,
                    v0_1,
                ))
            };
        }
        pub fn method185(
            v0_1: LrcPtr<Documents::UH4>,
            v1_1: List<LrcPtr<Documents::UH5>>,
        ) -> List<LrcPtr<Documents::UH5>> {
            match v0_1.as_ref() {
                Documents::UH4::UH4_0 => v1_1.clone(),
                Documents::UH4::UH4_1(v0_1_1_0, v0_1_1_1) => cons(
                    match v0_1.as_ref() {
                        Documents::UH4::UH4_1(x, _) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    Documents::method185(
                        match v0_1.as_ref() {
                            Documents::UH4::UH4_1(_, x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        v1_1.clone(),
                    ),
                ),
            }
        }
        pub fn method186(v0_1: i32, v1_1: LrcPtr<Documents::Mut9>) -> bool {
            (v1_1.l0.get().clone()) < (v0_1)
        }
        pub fn method187(
            v0_1: LrcPtr<Documents::UH5>,
            v1_1: List<(
                string,
                string,
                Func1<string, Func1<string, Documents::US32>>,
            )>,
        ) -> List<(
            string,
            string,
            Func1<string, Func1<string, Documents::US32>>,
        )> {
            match v0_1.as_ref() {
                Documents::UH5::UH5_0 => v1_1.clone(),
                Documents::UH5::UH5_1(v0_1_1_0, v0_1_1_1, v0_1_1_2, v0_1_1_3) => cons(
                    (
                        match v0_1.as_ref() {
                            Documents::UH5::UH5_1(x, _, _, _) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        match v0_1.as_ref() {
                            Documents::UH5::UH5_1(_, x, _, _) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        Func1::new({
                            let v0_1 = v0_1.clone();
                            move |a0: string| {
                                Func1::new({
                                    let a0 = a0.clone();
                                    let v0_1 = v0_1.clone();
                                    move |a1: string| {
                                        (match v0_1.as_ref() {
                                            Documents::UH5::UH5_1(_, _, x, _) => x.clone(),
                                            _ => unreachable!(),
                                        })(a0.clone(), a1)
                                    }
                                })
                            }
                        }),
                    ),
                    Documents::method187(
                        match v0_1.as_ref() {
                            Documents::UH5::UH5_1(_, _, _, x) => x.clone(),
                            _ => unreachable!(),
                        }
                        .clone(),
                        v1_1.clone(),
                    ),
                ),
            }
        }
        pub fn closure85(
            unitVar: (),
            _arg: (
                string,
                string,
                Func1<string, Func1<string, Documents::US32>>,
            ),
        ) -> Option<Result<string, LrcPtr<(string, string)>>> {
            let v4: Documents::US32 = (_arg.2.clone())(_arg.1.clone())(_arg.0.clone());
            match &v4 {
                Documents::US32::US32_0(v4_0_0) => Some(
                    match &v4 {
                        Documents::US32::US32_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                ),
                _ => None::<Result<string, LrcPtr<(string, string)>>>,
            }
        }
        pub fn method188(
            v0_1: Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
        ) -> Vec<Option<Result<string, LrcPtr<(string, string)>>>> {
            v0_1
        }
        pub fn method189(
            v0_1: Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
        ) -> Vec<Option<Result<string, LrcPtr<(string, string)>>>> {
            v0_1
        }
        pub fn closure33(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: string,
            v4: string,
            v5: string,
        ) -> Result<
            LrcPtr<(
                string,
                Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
            )>,
            std::string::String,
        > {
            let v6: string = Documents::method35(v5);
            let v9: &str = &*v6.clone();
            let v33: std::string::String = String::from(v9);
            let v57: std::path::PathBuf = std::path::PathBuf::from(v33);
            let v81: std::path::Display = v57.display();
            let v105: std::string::String = format!("{}", v81);
            let v134: string = concat(new_array(&[
                string("."),
                replace(
                    replace(
                        fable_library_rust::String_::fromString(v105),
                        v3.clone(),
                        Documents::method79(),
                    ),
                    string("\\"),
                    string("/"),
                ),
            ]));
            let v135: string = Documents::method36(v6);
            let v137: string = Documents::method80(Documents::method26(v2.clone(), v134.clone()));
            let patternInput: (i32, string) = Documents::method81(
                concat(new_array(&[
                    string("git ls-tree --format=\'%(objectname)\' origin/gh-pages \""),
                    v137.clone(),
                    string("\""),
                ])),
                None::<CancellationToken>,
                new_empty::<(string, string)>(),
                None::<Func1<(i32, string, bool), Arc<Async<()>>>>,
                None::<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
                true,
                Some(v2),
            );
            let v149: string = patternInput.1.clone();
            let v148: i32 = patternInput.0.clone();
            let v151: string = Documents::method80(Documents::method26(v3.clone(), v134.clone()));
            let patternInput_1: (i32, string) = Documents::method81(
                concat(new_array(&[
                    string("git hash-object \""),
                    v151.clone(),
                    string("\""),
                ])),
                None::<CancellationToken>,
                new_empty::<(string, string)>(),
                None::<Func1<(i32, string, bool), Arc<Async<()>>>>,
                None::<Func1<std::sync::Arc<std::sync::Mutex<std::process::ChildStdin>>, ()>>,
                true,
                Some(v3.clone()),
            );
            let v163: string = patternInput_1.1.clone();
            let v162: i32 = patternInput_1.0.clone();
            let v165: string = Documents::method80(Documents::method26(v4.clone(), v134.clone()));
            let v167: string =
                (Documents::method156(string("hangul.md"), v151.clone(), v4.clone()))
                    .0
                    .clone();
            let v628: LrcPtr<Documents::UH4> = if contains(v149.clone(), v163.clone()) {
                LrcPtr::new(Documents::UH4::UH4_0)
            } else {
                let v179: string = Documents::method36(v151.clone());
                let v181: Result<std::fs::File, std::io::Error> = std::fs::File::open(&*v179);
                let v184: std::fs::File = v181.unwrap();
                let v197: std::io::BufReader<std::fs::File> = std::io::BufReader::new(v184);
                let v199: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                    std::io::BufReader::new(v197);
                let v201: bool = true;
                let mut v199 = v199;
                let result: sha2::Sha256 = sha2::Digest::new();
                {
                    let v204: sha2::Sha256 = result;
                    let v206: bool = true;
                    let mut v204 = v204;
                    let v207: usize = 0_i32 as usize;
                    let v211: _ = [0_u8; 1024];
                    let v213: bool = true;
                    loop {
                        // rust.loop;
                        let v215: bool = true;
                        let mut v211 = v211;
                        let v217: Result<usize, std::io::Error> =
                            std::io::Read::read(&mut v199, &mut v211);
                        let v229: usize = v217.unwrap();
                        if (v229) == (v207) {
                            let v236: bool = true;
                            break;
                            ()
                        }
                        {
                            let v237: usize = v229;
                            let v252: &_ = if (v237) == (v211.len()) {
                                &v211[v207..]
                            } else {
                                &v211[v207..v237]
                            };
                            sha2::Digest::update(&mut v204, v252);
                            {
                                let v255: bool = true;
                            } // rust.loop;
                            let v257: bool = true;
                        } // rust.loop;
                        let v259: bool = true;
                    } // rust.loop;
                    let v261: bool = true;
                    {
                        // rust.loop;
                        let v263: bool = true;
                        {
                            // rust.loop;
                            let v265: &[u8] = &sha2::Digest::finalize(v204);
                            let v267: Vec<u8> = v265.iter().map(|x| *x).collect::<Vec<_>>();
                            let v269: bool = true;
                            let _vec_map: Vec<_> = v267
                                .into_iter()
                                .map(|x| {
                                    //;
                                    let v271: u8 = x;
                                    let v273: std::string::String = format!("{:02x}", v271);
                                    let v275: string =
                                        fable_library_rust::String_::fromString(v273);
                                    let v277: bool = true;
                                    v275
                                })
                                .collect::<Vec<_>>();
                            let v279: Vec<string> = _vec_map;
                            let v281: Array<string> =
                                fable_library_rust::NativeArray_::array_from(v279);
                            let _v282: LrcPtr<dyn IEnumerable_1<string>> = delay(Func0::new({
                                let v281 = v281.clone();
                                move || {
                                    map_1(
                                        Func1::new({
                                            let v281 = v281.clone();
                                            move |i: i32| v281[i].clone()
                                        }),
                                        rangeNumeric(
                                            0_i32,
                                            1_i32,
                                            (get_Count(v281.clone())) - 1_i32,
                                        ),
                                    )
                                }
                            }));
                            let v291: string = Documents::method157();
                            let v297: string = join(
                                if (v291.clone()) == string("\n") {
                                    Documents::method61(v291.clone())
                                } else {
                                    v291
                                },
                                toArray_1(_v282),
                            );
                            let v301 = Documents::method65();
                            let v304: Result<string, std::string::String> =
                                Ok::<string, std::io::Error>(v297).map_err(|x| v301(x));
                            let v316 = Documents::method158();
                            let v317 = Documents::method159();
                            let v319: Documents::US33 = match v304 {
                                Ok(x) => v316(x),
                                Err(e) => v317(e),
                            };
                            let v327: string = match &v319 {
                                Documents::US33::US33_0(v319_0_0) => v319_0_0.clone(),
                                Documents::US33::US33_1(v319_1_0) => panic!(
                                    "{}",
                                    sprintf!(
                                        "resultm.get / Result value was Error: {}",
                                        v319_1_0.clone()
                                    ),
                                ),
                            };
                            let v478: Documents::US5 = if (Documents::method160(v165.clone()))
                                == false
                            {
                                Documents::US5::US5_1
                            } else {
                                let v331: string = Documents::method36(v165.clone());
                                let v333: Result<std::fs::File, std::io::Error> =
                                    std::fs::File::open(&*v331);
                                let v336: std::fs::File = v333.unwrap();
                                let v349: std::io::BufReader<std::fs::File> =
                                    std::io::BufReader::new(v336);
                                let v351: std::io::BufReader<std::io::BufReader<std::fs::File>> =
                                    std::io::BufReader::new(v349);
                                let v353: bool = true;
                                let mut v351 = v351;
                                let result: sha2::Sha256 = sha2::Digest::new();
                                {
                                    let v356: sha2::Sha256 = result;
                                    let v358: bool = true;
                                    let mut v356 = v356;
                                    let v359: usize = 0_i32 as usize;
                                    let v363: _ = [0_u8; 1024];
                                    let v365: bool = true;
                                    loop {
                                        // rust.loop;
                                        let v367: bool = true;
                                        let mut v363 = v363;
                                        let v369: Result<usize, std::io::Error> =
                                            std::io::Read::read(&mut v351, &mut v363);
                                        let v381: usize = v369.unwrap();
                                        if (v381) == (v359) {
                                            let v388: bool = true;
                                            break;
                                            ()
                                        }
                                        {
                                            let v389: usize = v381;
                                            let v404: &_ = if (v389) == (v363.len()) {
                                                &v363[v359..]
                                            } else {
                                                &v363[v359..v389]
                                            };
                                            sha2::Digest::update(&mut v356, v404);
                                            {
                                                let v407: bool = true;
                                            } // rust.loop;
                                            let v409: bool = true;
                                        } // rust.loop;
                                        let v411: bool = true;
                                    } // rust.loop;
                                    let v413: bool = true;
                                    {
                                        // rust.loop;
                                        let v415: bool = true;
                                        {
                                            // rust.loop;
                                            let v417: &[u8] = &sha2::Digest::finalize(v356);
                                            let v419: Vec<u8> =
                                                v417.iter().map(|x| *x).collect::<Vec<_>>();
                                            let v421: bool = true;
                                            let _vec_map: Vec<_> = v419
                                                .into_iter()
                                                .map(|x| {
                                                    //;
                                                    let v423: u8 = x;
                                                    let v425: std::string::String =
                                                        format!("{:02x}", v423);
                                                    let v427: string =
                                                        fable_library_rust::String_::fromString(
                                                            v425,
                                                        );
                                                    let v429: bool = true;
                                                    v427
                                                })
                                                .collect::<Vec<_>>();
                                            let v431: Vec<string> = _vec_map;
                                            let v433: Array<string> =
                                                fable_library_rust::NativeArray_::array_from(v431);
                                            let _v434: LrcPtr<dyn IEnumerable_1<string>> =
                                                delay(Func0::new({
                                                    let v433 = v433.clone();
                                                    move || {
                                                        map_1(
                                                            Func1::new({
                                                                let v433 = v433.clone();
                                                                move |i_1: i32| v433[i_1].clone()
                                                            }),
                                                            rangeNumeric(
                                                                0_i32,
                                                                1_i32,
                                                                (get_Count(v433.clone())) - 1_i32,
                                                            ),
                                                        )
                                                    }
                                                }));
                                            let v443: string = Documents::method157();
                                            let v449: string = join(
                                                if (v443.clone()) == string("\n") {
                                                    Documents::method61(v443.clone())
                                                } else {
                                                    v443
                                                },
                                                toArray_1(_v434),
                                            );
                                            let v453 = Documents::method65();
                                            let v456: Result<string, std::string::String> =
                                                Ok::<string, std::io::Error>(v449)
                                                    .map_err(|x| v453(x));
                                            let v468 = Documents::method158();
                                            let v469 = Documents::method159();
                                            let v471: Documents::US33 = match v456 {
                                                Ok(x) => v468(x),
                                                Err(e) => v469(e),
                                            };
                                            match &v471 {
                                                Documents::US33::US33_0(v471_0_0) => {
                                                    Documents::US5::US5_0(v471_0_0.clone())
                                                }
                                                _ => Documents::US5::US5_1,
                                            }
                                        }
                                    }
                                }
                            };
                            if let Documents::US5::US5_0(v478_0_0) = &v478 {
                                if (v327.clone()) == (v478_0_0.clone()) {
                                    LrcPtr::new(Documents::UH4::UH4_0)
                                } else {
                                    let v484: () = {
                                        Documents::closure71(
                                            v134.clone(),
                                            v135.clone(),
                                            v137.clone(),
                                            v149.clone(),
                                            v148,
                                            v151.clone(),
                                            v163.clone(),
                                            v162,
                                            v165.clone(),
                                            v327.clone(),
                                            v478.clone(),
                                            (),
                                        );
                                        ()
                                    };
                                    Documents::method163(v165.clone(), v151.clone());
                                    LrcPtr::new(Documents::UH4::UH4_1(
                                        LrcPtr::new(Documents::UH5::UH5_1(
                                            string("hangul.md"),
                                            v151.clone(),
                                            Func2::new({
                                                let v0_1 = v0_1.clone();
                                                let v1_1 = v1_1.clone();
                                                let v3 = v3.clone();
                                                let v4 = v4.clone();
                                                move |b0: string, b1: string| {
                                                    (Func1::new({
                                                        let v0_1 = v0_1.clone();
                                                        let v1_1 = v1_1.clone();
                                                        let v3 = v3.clone();
                                                        let v4 = v4.clone();
                                                        move |v: string| {
                                                            Documents::closure76(
                                                                v4.clone(),
                                                                v3.clone(),
                                                                v1_1.clone(),
                                                                v0_1.clone(),
                                                                v,
                                                            )
                                                        }
                                                    }))(
                                                        b0
                                                    )(
                                                        b1
                                                    )
                                                }
                                            }),
                                            LrcPtr::new(Documents::UH5::UH5_0),
                                        )),
                                        LrcPtr::new(Documents::UH4::UH4_1(
                                            LrcPtr::new(Documents::UH5::UH5_1(
                                                string("html"),
                                                v151.clone(),
                                                Func2::new({
                                                    let v3 = v3.clone();
                                                    let v4 = v4.clone();
                                                    move |b0: string, b1: string| {
                                                        (Func1::new({
                                                            let v3 = v3.clone();
                                                            let v4 = v4.clone();
                                                            move |v_1: string| {
                                                                Documents::closure81(
                                                                    v4.clone(),
                                                                    v3.clone(),
                                                                    false,
                                                                    v_1,
                                                                )
                                                            }
                                                        }))(
                                                            b0
                                                        )(
                                                            b1
                                                        )
                                                    }
                                                }),
                                                LrcPtr::new(Documents::UH5::UH5_1(
                                                    string("pdf"),
                                                    v151.clone(),
                                                    Func2::new({
                                                        let v3 = v3.clone();
                                                        let v4 = v4.clone();
                                                        move |b0: string, b1: string| {
                                                            (Func1::new({
                                                                let v3 = v3.clone();
                                                                let v4 = v4.clone();
                                                                move |v_2: string| {
                                                                    Documents::closure81(
                                                                        v4.clone(),
                                                                        v3.clone(),
                                                                        false,
                                                                        v_2,
                                                                    )
                                                                }
                                                            }))(
                                                                b0
                                                            )(
                                                                b1
                                                            )
                                                        }
                                                    }),
                                                    LrcPtr::new(Documents::UH5::UH5_1(
                                                        string("epub"),
                                                        v151.clone(),
                                                        Func2::new({
                                                            let v3 = v3.clone();
                                                            let v4 = v4.clone();
                                                            move |b0: string, b1: string| {
                                                                (Func1::new({
                                                                    let v3 = v3.clone();
                                                                    let v4 = v4.clone();
                                                                    move |v_3: string| {
                                                                        Documents::closure81(
                                                                            v4.clone(),
                                                                            v3.clone(),
                                                                            false,
                                                                            v_3,
                                                                        )
                                                                    }
                                                                }))(
                                                                    b0
                                                                )(
                                                                    b1
                                                                )
                                                            }
                                                        }),
                                                        LrcPtr::new(Documents::UH5::UH5_1(
                                                            string("html"),
                                                            v167.clone(),
                                                            Func2::new({
                                                                let v3 = v3.clone();
                                                                let v4 = v4.clone();
                                                                move |b0: string, b1: string| {
                                                                    (Func1::new({
                                                                        let v3 = v3.clone();
                                                                        let v4 = v4.clone();
                                                                        move |v_4: string| {
                                                                            Documents::closure81(
                                                                                v4.clone(),
                                                                                v3.clone(),
                                                                                true,
                                                                                v_4,
                                                                            )
                                                                        }
                                                                    }))(
                                                                        b0
                                                                    )(
                                                                        b1
                                                                    )
                                                                }
                                                            }),
                                                            LrcPtr::new(Documents::UH5::UH5_1(
                                                                string("pdf"),
                                                                v167.clone(),
                                                                Func2::new({
                                                                    let v3 = v3.clone();
                                                                    let v4 = v4.clone();
                                                                    move |b0: string, b1: string| {
                                                                        (Func1::new({
                                                                            let v3 = v3.clone();
                                                                            let v4 = v4.clone();
                                                                            move |v_5: string| {
                                                                                Documents::closure81(
                                                                                    v4.clone(),
                                                                                    v3.clone(),
                                                                                    true,
                                                                                    v_5,
                                                                                )
                                                                            }
                                                                        }))(
                                                                            b0
                                                                        )(
                                                                            b1
                                                                        )
                                                                    }
                                                                }),
                                                                LrcPtr::new(Documents::UH5::UH5_1(
                                                                    string("epub"),
                                                                    v167.clone(),
                                                                    Func2::new({
                                                                        let v3 = v3.clone();
                                                                        let v4 = v4.clone();
                                                                        move
                                                                                                                                                                                                                                                                                                                                           |b0:
                                                                                                                                                                                                                                                                                                                                                string,
                                                                                                                                                                                                                                                                                                                                            b1:
                                                                                                                                                                                                                                                                                                                                                string|
                                                                                                                                                                                                                                                                                                                                           (Func1::new({
                                                                                                                                                                                                                                                                                                                                                           let v3
                                                                                                                                                                                                                                                                                                                                                               =
                                                                                                                                                                                                                                                                                                                                                               v3.clone();
                                                                                                                                                                                                                                                                                                                                                           let v4
                                                                                                                                                                                                                                                                                                                                                               =
                                                                                                                                                                                                                                                                                                                                                               v4.clone();
                                                                                                                                                                                                                                                                                                                                                           move
                                                                                                                                                                                                                                                                                                                                                               |v_6:
                                                                                                                                                                                                                                                                                                                                                                    string|
                                                                                                                                                                                                                                                                                                                                                               Documents::closure81(v4.clone(),
                                                                                                                                                                                                                                                                                                                                                                                    v3.clone(),
                                                                                                                                                                                                                                                                                                                                                                                    true,
                                                                                                                                                                                                                                                                                                                                                                                    v_6)
                                                                                                                                                                                                                                                                                                                                                       }))(b0)(b1)
                                                                    }),
                                                                    LrcPtr::new(
                                                                        Documents::UH5::UH5_0,
                                                                    ),
                                                                )),
                                                            )),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                            LrcPtr::new(Documents::UH4::UH4_0),
                                        )),
                                    ))
                                }
                            } else {
                                let v557: () = {
                                    Documents::closure71(
                                        v134,
                                        v135.clone(),
                                        v137,
                                        v149,
                                        v148,
                                        v151.clone(),
                                        v163,
                                        v162,
                                        v165.clone(),
                                        v327,
                                        v478.clone(),
                                        (),
                                    );
                                    ()
                                };
                                Documents::method163(v165, v151.clone());
                                LrcPtr::new(Documents::UH4::UH4_1(
                                    LrcPtr::new(Documents::UH5::UH5_1(
                                        string("hangul.md"),
                                        v151.clone(),
                                        Func2::new({
                                            let v0_1 = v0_1.clone();
                                            let v1_1 = v1_1.clone();
                                            let v3 = v3.clone();
                                            let v4 = v4.clone();
                                            move |b0: string, b1: string| {
                                                (Func1::new({
                                                    let v0_1 = v0_1.clone();
                                                    let v1_1 = v1_1.clone();
                                                    let v3 = v3.clone();
                                                    let v4 = v4.clone();
                                                    move |v_7: string| {
                                                        Documents::closure76(
                                                            v4.clone(),
                                                            v3.clone(),
                                                            v1_1.clone(),
                                                            v0_1.clone(),
                                                            v_7,
                                                        )
                                                    }
                                                }))(
                                                    b0
                                                )(b1)
                                            }
                                        }),
                                        LrcPtr::new(Documents::UH5::UH5_0),
                                    )),
                                    LrcPtr::new(Documents::UH4::UH4_1(
                                        LrcPtr::new(Documents::UH5::UH5_1(
                                            string("html"),
                                            v151.clone(),
                                            Func2::new({
                                                let v3 = v3.clone();
                                                let v4 = v4.clone();
                                                move |b0: string, b1: string| {
                                                    (Func1::new({
                                                        let v3 = v3.clone();
                                                        let v4 = v4.clone();
                                                        move |v_8: string| {
                                                            Documents::closure81(
                                                                v4.clone(),
                                                                v3.clone(),
                                                                false,
                                                                v_8,
                                                            )
                                                        }
                                                    }))(
                                                        b0
                                                    )(
                                                        b1
                                                    )
                                                }
                                            }),
                                            LrcPtr::new(Documents::UH5::UH5_1(
                                                string("pdf"),
                                                v151.clone(),
                                                Func2::new({
                                                    let v3 = v3.clone();
                                                    let v4 = v4.clone();
                                                    move |b0: string, b1: string| {
                                                        (Func1::new({
                                                            let v3 = v3.clone();
                                                            let v4 = v4.clone();
                                                            move |v_9: string| {
                                                                Documents::closure81(
                                                                    v4.clone(),
                                                                    v3.clone(),
                                                                    false,
                                                                    v_9,
                                                                )
                                                            }
                                                        }))(
                                                            b0
                                                        )(
                                                            b1
                                                        )
                                                    }
                                                }),
                                                LrcPtr::new(Documents::UH5::UH5_1(
                                                    string("epub"),
                                                    v151.clone(),
                                                    Func2::new({
                                                        let v3 = v3.clone();
                                                        let v4 = v4.clone();
                                                        move |b0: string, b1: string| {
                                                            (Func1::new({
                                                                let v3 = v3.clone();
                                                                let v4 = v4.clone();
                                                                move |v_10: string| {
                                                                    Documents::closure81(
                                                                        v4.clone(),
                                                                        v3.clone(),
                                                                        false,
                                                                        v_10,
                                                                    )
                                                                }
                                                            }))(
                                                                b0
                                                            )(
                                                                b1
                                                            )
                                                        }
                                                    }),
                                                    LrcPtr::new(Documents::UH5::UH5_1(
                                                        string("html"),
                                                        v167.clone(),
                                                        Func2::new({
                                                            let v3 = v3.clone();
                                                            let v4 = v4.clone();
                                                            move |b0: string, b1: string| {
                                                                (Func1::new({
                                                                    let v3 = v3.clone();
                                                                    let v4 = v4.clone();
                                                                    move |v_11: string| {
                                                                        Documents::closure81(
                                                                            v4.clone(),
                                                                            v3.clone(),
                                                                            true,
                                                                            v_11,
                                                                        )
                                                                    }
                                                                }))(
                                                                    b0
                                                                )(
                                                                    b1
                                                                )
                                                            }
                                                        }),
                                                        LrcPtr::new(Documents::UH5::UH5_1(
                                                            string("pdf"),
                                                            v167.clone(),
                                                            Func2::new({
                                                                let v3 = v3.clone();
                                                                let v4 = v4.clone();
                                                                move |b0: string, b1: string| {
                                                                    (Func1::new({
                                                                        let v3 = v3.clone();
                                                                        let v4 = v4.clone();
                                                                        move |v_12: string| {
                                                                            Documents::closure81(
                                                                                v4.clone(),
                                                                                v3.clone(),
                                                                                true,
                                                                                v_12,
                                                                            )
                                                                        }
                                                                    }))(
                                                                        b0
                                                                    )(
                                                                        b1
                                                                    )
                                                                }
                                                            }),
                                                            LrcPtr::new(Documents::UH5::UH5_1(
                                                                string("epub"),
                                                                v167.clone(),
                                                                Func2::new({
                                                                    let v3 = v3.clone();
                                                                    let v4 = v4.clone();
                                                                    move |b0: string, b1: string| {
                                                                        (Func1::new({
                                                                            let v3 = v3.clone();
                                                                            let v4 = v4.clone();
                                                                            move |v_13: string| {
                                                                                Documents::closure81(
                                                                                    v4.clone(),
                                                                                    v3.clone(),
                                                                                    true,
                                                                                    v_13,
                                                                                )
                                                                            }
                                                                        }))(
                                                                            b0
                                                                        )(
                                                                            b1
                                                                        )
                                                                    }
                                                                }),
                                                                LrcPtr::new(Documents::UH5::UH5_0),
                                                            )),
                                                        )),
                                                    )),
                                                )),
                                            )),
                                        )),
                                        LrcPtr::new(Documents::UH4::UH4_0),
                                    )),
                                ))
                            }
                        }
                    }
                }
            };
            let v1009: Array<LrcPtr<Documents::UH5>> = toArray(Documents::method185(
                if (if let Documents::UH4::UH4_0 = v628.as_ref() {
                    true
                } else {
                    false
                }) != true
                {
                    v628
                } else {
                    let patternInput_3: (string, string) =
                        Documents::method156(string("epub"), v167.clone(), v4.clone());
                    let v633: string = patternInput_3.1.clone();
                    let v632: string = patternInput_3.0.clone();
                    let v685: LrcPtr<Documents::UH5> = if if Documents::method160(v632.clone()) {
                        true
                    } else {
                        if (Documents::method160(v633.clone())) == false {
                            true
                        } else {
                            let v639: () = {
                                Documents::closure84(v633.clone(), v632.clone(), ());
                                ()
                            };
                            Documents::method163(v632, v633);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("epub"),
                            v167.clone(),
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_14: string| {
                                            Documents::closure81(v4.clone(), v3.clone(), true, v_14)
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            LrcPtr::new(Documents::UH5::UH5_0),
                        ))
                    } else {
                        LrcPtr::new(Documents::UH5::UH5_0)
                    };
                    let patternInput_4: (string, string) =
                        Documents::method156(string("pdf"), v167.clone(), v4.clone());
                    let v688: string = patternInput_4.1.clone();
                    let v687: string = patternInput_4.0.clone();
                    let v738: LrcPtr<Documents::UH5> = if if Documents::method160(v687.clone()) {
                        true
                    } else {
                        if (Documents::method160(v688.clone())) == false {
                            true
                        } else {
                            let v694: () = {
                                Documents::closure84(v688.clone(), v687.clone(), ());
                                ()
                            };
                            Documents::method163(v687, v688);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("pdf"),
                            v167.clone(),
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_15: string| {
                                            Documents::closure81(v4.clone(), v3.clone(), true, v_15)
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            v685.clone(),
                        ))
                    } else {
                        v685
                    };
                    let patternInput_5: (string, string) =
                        Documents::method156(string("html"), v167.clone(), v4.clone());
                    let v741: string = patternInput_5.1.clone();
                    let v740: string = patternInput_5.0.clone();
                    let v791: LrcPtr<Documents::UH5> = if if Documents::method160(v740.clone()) {
                        true
                    } else {
                        if (Documents::method160(v741.clone())) == false {
                            true
                        } else {
                            let v747: () = {
                                Documents::closure84(v741.clone(), v740.clone(), ());
                                ()
                            };
                            Documents::method163(v740, v741);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("html"),
                            v167,
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_16: string| {
                                            Documents::closure81(v4.clone(), v3.clone(), true, v_16)
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            v738.clone(),
                        ))
                    } else {
                        v738
                    };
                    let patternInput_6: (string, string) =
                        Documents::method156(string("epub"), v151.clone(), v4.clone());
                    let v793: string = patternInput_6.1.clone();
                    let v792: string = patternInput_6.0.clone();
                    let v843: LrcPtr<Documents::UH5> = if if Documents::method160(v792.clone()) {
                        true
                    } else {
                        if (Documents::method160(v793.clone())) == false {
                            true
                        } else {
                            let v799: () = {
                                Documents::closure84(v793.clone(), v792.clone(), ());
                                ()
                            };
                            Documents::method163(v792, v793);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("epub"),
                            v151.clone(),
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_17: string| {
                                            Documents::closure81(
                                                v4.clone(),
                                                v3.clone(),
                                                false,
                                                v_17,
                                            )
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            v791.clone(),
                        ))
                    } else {
                        v791
                    };
                    let patternInput_7: (string, string) =
                        Documents::method156(string("pdf"), v151.clone(), v4.clone());
                    let v845: string = patternInput_7.1.clone();
                    let v844: string = patternInput_7.0.clone();
                    let v895: LrcPtr<Documents::UH5> = if if Documents::method160(v844.clone()) {
                        true
                    } else {
                        if (Documents::method160(v845.clone())) == false {
                            true
                        } else {
                            let v851: () = {
                                Documents::closure84(v845.clone(), v844.clone(), ());
                                ()
                            };
                            Documents::method163(v844, v845);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("pdf"),
                            v151.clone(),
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_18: string| {
                                            Documents::closure81(
                                                v4.clone(),
                                                v3.clone(),
                                                false,
                                                v_18,
                                            )
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            v843.clone(),
                        ))
                    } else {
                        v843
                    };
                    let patternInput_8: (string, string) =
                        Documents::method156(string("html"), v151.clone(), v4.clone());
                    let v897: string = patternInput_8.1.clone();
                    let v896: string = patternInput_8.0.clone();
                    let v947: LrcPtr<Documents::UH5> = if if Documents::method160(v896.clone()) {
                        true
                    } else {
                        if (Documents::method160(v897.clone())) == false {
                            true
                        } else {
                            let v903: () = {
                                Documents::closure84(v897.clone(), v896.clone(), ());
                                ()
                            };
                            Documents::method163(v896, v897);
                            false
                        }
                    } {
                        LrcPtr::new(Documents::UH5::UH5_1(
                            string("html"),
                            v151.clone(),
                            Func2::new({
                                let v3 = v3.clone();
                                let v4 = v4.clone();
                                move |b0: string, b1: string| {
                                    (Func1::new({
                                        let v3 = v3.clone();
                                        let v4 = v4.clone();
                                        move |v_19: string| {
                                            Documents::closure81(
                                                v4.clone(),
                                                v3.clone(),
                                                false,
                                                v_19,
                                            )
                                        }
                                    }))(b0)(b1)
                                }
                            }),
                            v895.clone(),
                        ))
                    } else {
                        v895
                    };
                    let patternInput_9: (string, string) =
                        Documents::method156(string("hangul.md"), v151.clone(), v4.clone());
                    let v949: string = patternInput_9.1.clone();
                    let v948: string = patternInput_9.0.clone();
                    LrcPtr::new(Documents::UH4::UH4_1(
                        if if Documents::method160(v948.clone()) {
                            true
                        } else {
                            if (Documents::method160(v949.clone())) == false {
                                true
                            } else {
                                let v955: () = {
                                    Documents::closure84(v949.clone(), v948.clone(), ());
                                    ()
                                };
                                Documents::method163(v948, v949);
                                false
                            }
                        } {
                            LrcPtr::new(Documents::UH5::UH5_1(
                                string("hangul.md"),
                                v151,
                                Func2::new({
                                    let v0_1 = v0_1.clone();
                                    let v1_1 = v1_1.clone();
                                    let v3 = v3.clone();
                                    let v4 = v4.clone();
                                    move |b0: string, b1: string| {
                                        (Func1::new({
                                            let v0_1 = v0_1.clone();
                                            let v1_1 = v1_1.clone();
                                            let v3 = v3.clone();
                                            let v4 = v4.clone();
                                            move |v_20: string| {
                                                Documents::closure76(
                                                    v4.clone(),
                                                    v3.clone(),
                                                    v1_1.clone(),
                                                    v0_1.clone(),
                                                    v_20,
                                                )
                                            }
                                        }))(b0)(b1)
                                    }
                                }),
                                LrcPtr::new(Documents::UH5::UH5_0),
                            ))
                        } else {
                            LrcPtr::new(Documents::UH5::UH5_0)
                        },
                        LrcPtr::new(Documents::UH4::UH4_1(
                            v947,
                            LrcPtr::new(Documents::UH4::UH4_0),
                        )),
                    ))
                },
                empty::<LrcPtr<Documents::UH5>>(),
            ));
            let v1013: Vec<LrcPtr<Documents::UH5>> = v1009.to_vec();
            let v1016: Vec<Option<Result<string, LrcPtr<(string, string)>>>> =
                new_empty::<Option<Result<string, LrcPtr<(string, string)>>>>().to_vec();
            let v1018: Array<LrcPtr<Documents::UH5>> =
                fable_library_rust::NativeArray_::array_from(v1013);
            let v1019: i32 = get_Count(v1018.clone());
            let v1020: LrcPtr<Documents::Mut9> = LrcPtr::new(Documents::Mut9 {
                l0: MutCell::new(0_i32),
                l1: MutCell::new(v1016),
            });
            while Documents::method186(v1019, v1020.clone()) {
                let v1022: i32 = v1020.l0.get().clone();
                let v1023: Vec<Option<Result<string, LrcPtr<(string, string)>>>> =
                    v1020.l1.get().clone();
                let v1028 = toArray(Documents::method187(
                    v1018[v1022].clone(),
                    empty::<(
                        string,
                        string,
                        Func1<string, Func1<string, Documents::US32>>,
                    )>(),
                ));
                let v1032 = v1028.to_vec();
                let v1034 = rayon::iter::IntoParallelIterator::into_par_iter(v1032);
                let v1037 = rayon::iter::ParallelIterator::map(v1034, |x| {
                    Func1::new(
                        move |arg10_0040_9: (
                            string,
                            string,
                            Func1<string, Func1<string, Documents::US32>>,
                        )| Documents::closure85((), arg10_0040_9),
                    )(x)
                });
                let v1040: Vec<Option<Result<string, LrcPtr<(string, string)>>>> =
                    Documents::method188(rayon::iter::ParallelIterator::collect(v1037));
                let v1041: Vec<Option<Result<string, LrcPtr<(string, string)>>>> =
                    Documents::method189(v1023);
                let v1043: bool = true;
                let mut v1041 = v1041;
                let v1045: bool = true;
                v1041.extend(v1040);
                let v1047: Vec<Option<Result<string, LrcPtr<(string, string)>>>> = v1041;
                let v1048: i32 = (v1022) + 1_i32;
                v1020.l0.set(v1048);
                v1020.l1.set(v1047);
                ()
            }
            Ok::<
                LrcPtr<(
                    string,
                    Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                )>,
                std::string::String,
            >(LrcPtr::new((v135, v1020.l1.get().clone())))
        }
        pub fn method190(
            v0_1: Result<
                Vec<
                    Result<
                        LrcPtr<(
                            string,
                            Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                        )>,
                        std::string::String,
                    >,
                >,
                std::string::String,
            >,
        ) -> Result<
            Vec<
                Result<
                    LrcPtr<(
                        string,
                        Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                    )>,
                    std::string::String,
                >,
            >,
            std::string::String,
        > {
            v0_1
        }
        pub fn method25(
            v0_1: string,
            v1_1: string,
            v2: string,
            v3: string,
        ) -> std::pin::Pin<
            Box<
                dyn std::future::Future<
                    Output = Result<
                        Vec<
                            Result<
                                LrcPtr<(
                                    string,
                                    Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                                )>,
                                std::string::String,
                            >,
                        >,
                        std::string::String,
                    >,
                >,
            >,
        > {
            let v8: Documents::US7 = Documents::method27(
                Documents::method26(string("polyglot"), string(".devcontainer")),
                string("C:\\home\\git\\polyglot\\target\\Builder\\documents"),
            );
            let v56: Documents::US5 = match &v8 {
                Documents::US7::US7_0(v8_0_0) => Documents::US5::US5_0(v8_0_0.clone()),
                Documents::US7::US7_1(v8_1_0) => {
                    let v14: () = {
                        Documents::closure13(v8_1_0.clone(), ());
                        ()
                    };
                    Documents::US5::US5_1
                }
            };
            let v111: Documents::US5 = match &v56 {
                Documents::US5::US5_0(v56_0_0) => Documents::US5::US5_0(
                    match &v56 {
                        Documents::US5::US5_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                ),
                _ => {
                    let v59: string = Documents::method34();
                    let v61: Documents::US7 = Documents::method27(
                        Documents::method26(string("polyglot"), string(".devcontainer")),
                        v59,
                    );
                    match &v61 {
                        Documents::US7::US7_0(v61_0_0) => Documents::US5::US5_0(v61_0_0.clone()),
                        Documents::US7::US7_1(v61_1_0) => {
                            let v67: () = {
                                Documents::closure13(v61_1_0.clone(), ());
                                ()
                            };
                            Documents::US5::US5_1
                        }
                    }
                }
            };
            let v116: string = Documents::method26(
                match &v111 {
                    Documents::US5::US5_0(v111_0_0) => match &v111 {
                        Documents::US5::US5_0(x) => x.clone(),
                        _ => unreachable!(),
                    }
                    .clone(),
                    _ => panic!("{}", string("Option does not have a value."),),
                },
                string("polyglot"),
            );
            let v117: string = Documents::method35(v3);
            let v118: string = Documents::method35(v2);
            let v119: string = Documents::method35(v1_1);
            let v122: () = {
                Documents::closure22(v0_1.clone(), v117.clone(), v118.clone(), v119.clone(), ());
                ()
            };
            let v163: bool = true;
            let __future_init = Box::pin(async move {
                //;
                let v165: async_walkdir::WalkDir = async_walkdir::WalkDir::new(&*v118.clone());
                let v168: async_walkdir::WalkDir = async_walkdir::WalkDir::filter(v165, move |x| {
                    Func1::new(move |v: async_walkdir::DirEntry| Documents::closure23((), v))(x)
                });
                let v169 = Documents::method70();
                let v171 = futures::stream::StreamExt::filter_map(v168, |x| async { v169(x) });
                let v173: std::pin::Pin<Box<dyn std::future::Future<Output = Vec<string>>>> =
                    Box::pin(futures::stream::StreamExt::collect(v171));
                let v175: Vec<string> = v173.await;
                let v178: () = {
                    Documents::closure32(v175.clone(), ());
                    ()
                };
                let v221: rayon::vec::IntoIter<string> =
                    rayon::iter::IntoParallelIterator::into_par_iter(v175);
                let v224: rayon::iter::Map<rayon::vec::IntoIter<string>, _> =
                    rayon::iter::ParallelIterator::map(v221, |x| {
                        Func1::new({
                            let v0_1 = v0_1.clone();
                            let v116 = v116.clone();
                            let v117 = v117.clone();
                            let v118 = v118.clone();
                            let v119 = v119.clone();
                            move |v_1: string| {
                                Documents::closure33(
                                    v0_1.clone(),
                                    v116.clone(),
                                    v117.clone(),
                                    v118.clone(),
                                    v119.clone(),
                                    v_1,
                                )
                            }
                        })(x)
                    });
                let v228: Result<
                    Vec<
                        Result<
                            LrcPtr<(
                                string,
                                Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                            )>,
                            std::string::String,
                        >,
                    >,
                    std::string::String,
                > = Documents::method190(Ok::<
                    Vec<
                        Result<
                            LrcPtr<(
                                string,
                                Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                            )>,
                            std::string::String,
                        >,
                    >,
                    std::string::String,
                >(rayon::iter::ParallelIterator::collect(
                    v224,
                )));
                let v231: string = string("}");
                let v235: bool = true;
                let v232 = v228;
                let v247: string = append(
                    (append(
                        (append((append(string("true; v232 "), (v231))), string("); "))),
                        string(""),
                    )),
                    string(" // rust.fix_closure\'"),
                );
                let v248: bool = true;
                v232
            }); // rust.fix_closure';
            let v250 = __future_init;
            v250
        }
        pub fn closure86(
            unitVar: (),
            v0_1: Vec<
                Result<
                    LrcPtr<(
                        string,
                        Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                    )>,
                    std::string::String,
                >,
            >,
        ) -> Documents::US36 {
            Documents::US36::US36_0(v0_1)
        }
        pub fn method191() -> Func1<
            Vec<
                Result<
                    LrcPtr<(
                        string,
                        Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                    )>,
                    std::string::String,
                >,
            >,
            Documents::US36,
        > {
            Func1::new(
                move |v: Vec<
                    Result<
                        LrcPtr<(
                            string,
                            Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                        )>,
                        std::string::String,
                    >,
                >| Documents::closure86((), v),
            )
        }
        pub fn closure87(unitVar: (), v0_1: std::string::String) -> Documents::US36 {
            Documents::US36::US36_1(v0_1)
        }
        pub fn method192() -> Func1<std::string::String, Documents::US36> {
            Func1::new(move |v: std::string::String| Documents::closure87((), v))
        }
        pub fn method193(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: std::string::String,
        ) -> string {
            let v9: string = Documents::method76(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.main"),
                v9
            ))
        }
        pub fn closure88(v0_1: std::string::String, unitVar: ()) {
            if Documents::method7(Documents::US0::US0_4) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method193(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method74(),
                    v0_1,
                ))
            };
        }
        pub fn method195(v0_1: usize) -> string {
            let v2: LrcPtr<Documents::Mut3> = LrcPtr::new(Documents::Mut3 {
                l0: MutCell::new(Documents::method14()),
            });
            let v9: () = {
                Documents::closure8(v2.clone(), string("{ "), ());
                ()
            };
            let v18: () = {
                Documents::closure8(v2.clone(), string("result_len"), ());
                ()
            };
            let v27: () = {
                Documents::closure8(v2.clone(), string(" = "), ());
                ()
            };
            let v32: std::string::String = format!("{:#?}", v0_1);
            let v65: () = {
                Documents::closure8(v2.clone(), fable_library_rust::String_::fromString(v32), ());
                ()
            };
            let v74: () = {
                Documents::closure8(v2.clone(), string(" }"), ());
                ()
            };
            v2.l0.get().clone()
        }
        pub fn method194(
            v0_1: LrcPtr<Documents::Mut0>,
            v1_1: LrcPtr<Documents::Mut1>,
            v2: LrcPtr<Documents::Mut2>,
            v3: LrcPtr<Documents::Mut3>,
            v4: LrcPtr<Documents::Mut4>,
            v5: Option<i64>,
            v6: string,
            v7: string,
            v8: usize,
        ) -> string {
            let v9: string = Documents::method195(v8);
            Documents::method18(sprintf!(
                "{} {} #{} {} / {}",
                v6,
                v7,
                v0_1.l0.get().clone(),
                string("documents.main"),
                v9
            ))
        }
        pub fn closure89(
            v0_1: Vec<
                Result<
                    LrcPtr<(
                        string,
                        Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                    )>,
                    std::string::String,
                >,
            >,
            unitVar: (),
        ) {
            if Documents::method7(Documents::US0::US0_2) {
                let v5: () = {
                    Documents::closure6((), ());
                    ()
                };
                let patternInput: (
                    LrcPtr<Documents::Mut0>,
                    LrcPtr<Documents::Mut1>,
                    LrcPtr<Documents::Mut2>,
                    LrcPtr<Documents::Mut3>,
                    LrcPtr<Documents::Mut4>,
                    Option<i64>,
                ) = Documents::TraceState::trace_state().get().clone().unwrap();
                let v24: Option<i64> = patternInput.5.clone();
                let v23: LrcPtr<Documents::Mut4> = patternInput.4.clone();
                let v22: LrcPtr<Documents::Mut3> = patternInput.3.clone();
                let v21: LrcPtr<Documents::Mut2> = patternInput.2.clone();
                let v20: LrcPtr<Documents::Mut1> = patternInput.1.clone();
                let v19: LrcPtr<Documents::Mut0> = patternInput.0.clone();
                Documents::method19(Documents::method194(
                    v19.clone(),
                    v20.clone(),
                    v21.clone(),
                    v22.clone(),
                    v23.clone(),
                    v24.clone(),
                    Documents::method8(v19, v20, v21, v22, v23, v24),
                    Documents::method12(),
                    v0_1.len(),
                ))
            };
        }
        pub fn closure1(unitVar: (), v0_1: Array<string>) -> i32 {
            let v3: () = {
                Documents::closure2((), ());
                ()
            };
            let v19: () = {
                Documents::closure5(v0_1, ());
                ()
            };
            let v59: clap::Command = Documents::method0();
            let v61: clap::ArgMatches = clap::Command::get_matches(v59);
            let v62: string = Documents::method20();
            let v65: &str = &*v62;
            let v88: Option<std::string::String> =
                clap::ArgMatches::get_one(&v61.clone(), v65).cloned();
            let v102: Documents::US6 =
                defaultValue(Documents::US6::US6_1, map(Documents::method21(), v88));
            let v109: std::string::String = match &v102 {
                Documents::US6::US6_0(v102_0_0) => match &v102 {
                    Documents::US6::US6_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => panic!("{}", string("Option does not have a value."),),
            };
            let v111: string = fable_library_rust::String_::fromString(v109);
            let v112: string = Documents::method22();
            let v115: &str = &*v112;
            let v138: Option<std::string::String> =
                clap::ArgMatches::get_one(&v61.clone(), v115).cloned();
            let v152: Documents::US6 =
                defaultValue(Documents::US6::US6_1, map(Documents::method21(), v138));
            let v159: std::string::String = match &v152 {
                Documents::US6::US6_0(v152_0_0) => match &v152 {
                    Documents::US6::US6_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => panic!("{}", string("Option does not have a value."),),
            };
            let v161: string = fable_library_rust::String_::fromString(v159);
            let v162: string = Documents::method23();
            let v165: &str = &*v162;
            let v188: Option<std::string::String> =
                clap::ArgMatches::get_one(&v61.clone(), v165).cloned();
            let v202: Documents::US6 =
                defaultValue(Documents::US6::US6_1, map(Documents::method21(), v188));
            let v209: std::string::String = match &v202 {
                Documents::US6::US6_0(v202_0_0) => match &v202 {
                    Documents::US6::US6_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => panic!("{}", string("Option does not have a value."),),
            };
            let v211: string = fable_library_rust::String_::fromString(v209);
            let v212: string = Documents::method24();
            let v215: &str = &*v212;
            let v238: Option<std::string::String> = clap::ArgMatches::get_one(&v61, v215).cloned();
            let v252: Documents::US6 =
                defaultValue(Documents::US6::US6_1, map(Documents::method21(), v238));
            let v259: std::string::String = match &v252 {
                Documents::US6::US6_0(v252_0_0) => match &v252 {
                    Documents::US6::US6_0(x) => x.clone(),
                    _ => unreachable!(),
                }
                .clone(),
                _ => panic!("{}", string("Option does not have a value."),),
            };
            let v262: std::pin::Pin<
                Box<
                    dyn std::future::Future<
                        Output = Result<
                            Vec<
                                Result<
                                    LrcPtr<(
                                        string,
                                        Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                                    )>,
                                    std::string::String,
                                >,
                            >,
                            std::string::String,
                        >,
                    >,
                >,
            > = Documents::method25(
                fable_library_rust::String_::fromString(v259),
                v211,
                v161,
                v111,
            );
            let v264: Result<
                Vec<
                    Result<
                        LrcPtr<(
                            string,
                            Vec<Option<Result<string, LrcPtr<(string, string)>>>>,
                        )>,
                        std::string::String,
                    >,
                >,
                std::string::String,
            > = futures::executor::block_on(v262);
            let v265 = Documents::method191();
            let v266 = Documents::method192();
            let v267: Documents::US36 = match &v264 {
                Err(v264_1_0) => v266(v264_1_0.clone()),
                Ok(v264_0_0) => v265(v264_0_0.clone()),
            };
            match &v267 {
                Documents::US36::US36_0(v267_0_0) => {
                    let v271: () = {
                        Documents::closure89(v267_0_0.clone(), ());
                        ()
                    };
                    0_i32
                }
                Documents::US36::US36_1(v267_1_0) => {
                    let v316: () = {
                        Documents::closure88(v267_1_0.clone(), ());
                        ()
                    };
                    1_i32
                }
            }
        }
        pub fn v0() -> Func0<()> {
            static v0: OnceInit<Func0<()>> = OnceInit::new();
            v0.get_or_init(|| Func0::new(move || Documents::closure0((), ())))
                .clone()
        }
        pub fn tests() {
            (Documents::v0())();
        }
        pub fn v1() -> Func1<Array<string>, i32> {
            static v1: OnceInit<Func1<Array<string>, i32>> = OnceInit::new();
            v1.get_or_init(|| Func1::new(move |v: Array<string>| Documents::closure1((), v)))
                .clone()
        }
        pub fn main(args: Array<string>) -> i32 {
            (Documents::v1())(args)
        }
    }
}
pub use module_2555ccf7::*;
#[path = "../../../polyglot/lib/fsharp/Common.rs"]
mod module_ad43931;
pub use module_ad43931::*;
#[path = "../../../polyglot/lib/spiral/async_.rs"]
mod module_67c461a2;
pub use module_67c461a2::*;
#[path = "../../../polyglot/lib/spiral/common.rs"]
mod module_181b15d6;
pub use module_181b15d6::*;
#[path = "../../../polyglot/lib/spiral/crypto.rs"]
mod module_90d9c778;
pub use module_90d9c778::*;
#[path = "../../../polyglot/lib/spiral/date_time.rs"]
mod module_e43a8385;
pub use module_e43a8385::*;
#[path = "../../../polyglot/lib/spiral/file_system.rs"]
mod module_a7db9b47;
pub use module_a7db9b47::*;
#[path = "../../../polyglot/lib/spiral/lib.rs"]
mod module_98e448fc;
pub use module_98e448fc::*;
#[path = "../../../polyglot/lib/spiral/networking.rs"]
mod module_268024e5;
pub use module_268024e5::*;
#[path = "../../../polyglot/lib/spiral/platform.rs"]
mod module_7d8ad484;
pub use module_7d8ad484::*;
#[path = "../../../polyglot/lib/spiral/runtime.rs"]
mod module_485aae07;
pub use module_485aae07::*;
#[path = "../../../polyglot/lib/spiral/sm.rs"]
mod module_582b4305;
pub use module_582b4305::*;
#[path = "../../../polyglot/lib/spiral/threading.rs"]
mod module_d5afb6f5;
pub use module_d5afb6f5::*;
#[path = "../../../polyglot/lib/spiral/trace.rs"]
mod module_9e77af3a;
pub use module_9e77af3a::*;
pub mod Polyglot {
    pub use crate::module_ad43931::Polyglot::*;
}
pub fn main() {
    let args = std::env::args().skip(1).map(fromString).collect();
    Documents::main(array_from(args));
}
