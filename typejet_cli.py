import argparse
import sys
import time
import pyperclip
import typejet

def main():
    parser = argparse.ArgumentParser(
        description="TypeJet: Simulate realistic human keyboard input from clipboard or text."
    )
    parser.add_argument(
        "--mode",
        choices=["code", "raw"],
        default="code",
        help="Formatting mode: 'code' strips indentation/whitespace from each line to avoid editor auto-indent conflicts (default), 'raw' types exactly as copied."
    )
    parser.add_argument(
        "--delay",
        type=float,
        default=3.0,
        help="Initial countdown delay in seconds to let you focus the target window (default: 3.0)"
    )
    parser.add_argument(
        "--text",
        type=str,
        default=None,
        help="Direct text to type instead of reading from clipboard."
    )
    parser.add_argument(
        "--char-min",
        type=float,
        default=0.002,
        help="Minimum delay between keystrokes in seconds (default: 0.002)"
    )
    parser.add_argument(
        "--char-max",
        type=float,
        default=0.003,
        help="Maximum delay between keystrokes in seconds (default: 0.003)"
    )
    parser.add_argument(
        "--dwell-min",
        type=int,
        default=20,
        help="Minimum key press dwell time in milliseconds (default: 20)"
    )
    parser.add_argument(
        "--dwell-max",
        type=int,
        default=60,
        help="Maximum key press dwell time in milliseconds (default: 60)"
    )
    parser.add_argument(
        "--shift-min",
        type=int,
        default=10,
        help="Minimum shift key press delay in milliseconds (default: 10)"
    )
    parser.add_argument(
        "--shift-max",
        type=int,
        default=30,
        help="Maximum shift key press delay in milliseconds (default: 30)"
    )

    args = parser.parse_args()

    # Get input content
    if args.text is not None:
        content = args.text
    else:
        content = pyperclip.paste()

    if not content:
        print("Error: Input text is empty. Clipboard contains no text and no --text argument was provided.")
        sys.exit(1)

    # Process content based on mode
    if args.mode == "code":
        # Normalize Windows CRLF line endings to LF, split, strip lines, and rejoin
        lines = content.replace("\r\n", "\n").split("\n")
        stripped = [line.strip() for line in lines]
        content = "\n".join(stripped)

    # Print countdown to let user focus the target editor/window
    if args.delay > 0:
        print(f"Ready to type. You have {args.delay} seconds to focus your target window/input field...")
        remaining = args.delay
        while remaining > 0:
            print(f"Starting in {remaining:.1f}s...", end="\r", flush=True)
            sleep_step = min(0.5, remaining)
            time.sleep(sleep_step)
            remaining -= sleep_step
        print("Starting now!              ")
    else:
        print("Starting typing immediately...")

    try:
        typejet.type_text(
            content,
            min_char_delay=args.char_min,
            max_char_delay=args.char_max,
            min_dwell_time=args.dwell_min,
            max_dwell_time=args.dwell_max,
            min_shift_delay=args.shift_min,
            max_shift_delay=args.shift_max
        )
        print("Successfully typed content.")
    except Exception as e:
        print(f"Error during typing simulation: {e}")
        sys.exit(1)

if __name__ == "__main__":
    main()
