#include <stdio.h>
#include <string.h>

#include "winit.h"

typedef struct UserData {
  Window* window;
} UserData;

WinitControlFlow process_event(void* data, WinitEvent* event) {
    // rust owns `event`, no need to drop

    printf("New event\n");

    UserData* user_data = (UserData*) data;

    if (event -> event_type == WinitEventType_WindowEventCloseRequested) {
        return WinitControlFlow_Exit;
    }

    if (event -> event_type == WinitEventType_WindowEventResized) {
        printf("Window resized to: (%d x %d)\n", event -> window_resized.width, event -> window_resized.height);
    }

    if (event -> event_type == WinitEventType_WindowEventCursorMoved) {
        printf("Cursor moved to: (%f @ %f)\n", event -> cursor_moved.x, event -> cursor_moved.y);
    }

    if (event -> event_type == WinitEventType_WindowEventScaleFactorChanged) {
        printf("Scale factor changed to: %f. New physical size: (%d x %d)\n", event -> scale_factor.scale_factor, event -> scale_factor.width, event -> scale_factor.height);
    }

    if (event -> event_type == WinitEventType_MainEventsCleared) {
        //winit_window_request_redraw(user_data -> window);
    }

    return WinitControlFlow_Wait;
}

void with_title(WindowBuilder* window_builder, char* title) {
    StringBox* title_string = boxer_string_from_byte_string((uint8_t*) title, strlen(title));
    winit_window_builder_with_title(window_builder, title_string);
    boxer_string_drop(title_string);
}

int main() {
    EventLoop* event_loop = winit_event_loop_new();

    WindowBuilder* window_builder = winit_window_builder_new();
    with_title(window_builder, "Hello World");
    // Default logical window size
    winit_window_builder_with_dimensions(window_builder, 600.0, 400.0);

    Window* window = winit_create_window(event_loop, window_builder);
    // don't forget to drop the window builder!
    winit_window_builder_drop(window_builder);

    struct UserData user_data;
    user_data.window = window;

    // run the event loop, will continue if `process_event` returns WinitControlFlow_Exit
    winit_event_loop_run_return_data(event_loop, (void*)&user_data, process_event);

    winit_window_drop(window);
    winit_event_loop_drop(event_loop);

    return 0;
}
