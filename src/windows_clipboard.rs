/*
Copyright 2016 Avraham Weinstock

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use clipboard_win::{get_clipboard_string, set_clipboard_string};

use common::{ClipboardProvider, ClipboardContent, ImageData};
use std::error::Error;

pub struct WindowsClipboardContext;

impl ClipboardProvider for WindowsClipboardContext {
    fn new() -> Result<Self, Box<Error>> {
        Ok(WindowsClipboardContext)
    }
    fn get_text(&mut self) -> Result<String, Box<Error>> {
        Ok(get_clipboard_string()?)
    }
    fn set_text(&mut self, data: String) -> Result<(), Box<Error>> {
        Ok(set_clipboard_string(&data)?)
    }
    fn get_binary_contents(&mut self) -> Result<Option<ClipboardContent>, Box<Error>> {
        Err("get_binary_contents is not yet implemented for windows.".into())
    }
    fn get_image(&mut self) -> Result<ImageData, Box<Error>> {
        Err("get_binary_contents is not yet implemented for windows.".into())
    }
}
