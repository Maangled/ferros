import json
import requests
import sys

def main():
    try:
        with open('state.json', 'r') as f:
            state = json.load(f)
    except Exception as e:
        print(f"Error loading state.json: {e}")
        return

    open_chats = state.get('openChats', [])
    packets = state.get('packets', [])
    archived_ids = []
    
    for chat in open_chats:
        chat_id = chat['id']
        label = chat['label']
        
        # Rule 1: label starts with "User -> FERROS #" OR "User x FERROS #"
        if not (label.startswith("User -> FERROS #") or label.startswith("User x FERROS #")):
            continue
            
        messages = chat.get('messages', [])
        if not messages:
            continue
            
        user_msgs = [m for m in messages if m.get('role') == 'user']
        # The agent messages might have roles like 'assistant', 'agent', or 'system' depending on implementation
        # Usually in these logs it's 'assistant' or 'FERROS Agent' might be in the content
        agent_msgs = [m for m in messages if m.get('role') != 'user']

        if not user_msgs or not agent_msgs:
            continue
            
        latest_user_msg = user_msgs[-1].get('content', '').strip()
        latest_agent_msg = agent_msgs[-1].get('content', '')
        
        # Rule 2: latest user message is exactly "please route to core" OR exactly "please route to software"
        if latest_user_msg not in ["please route to core", "please route to software"]:
            continue
            
        # Rule 3: latest FERROS Agent message contains "completed child packet" OR "Recorded execution evidence will advance on the manager loop"
        if not ("completed child packet" in latest_agent_msg or "Recorded execution evidence will advance on the manager loop" in latest_agent_msg):
            continue
            
        # Rule 4: no packet for that session is in state human_intervention_required
        session_packets = [p for p in packets if p.get('sessionId') == chat_id]
        if any(p.get('state') == 'human_intervention_required' for p in session_packets):
            continue
            
        # Archive
        try:
            resp = requests.post(f"http://localhost:4317/monitor/sessions/{chat_id}/archive")
            if resp.status_code == 200:
                archived_ids.append(chat_id)
            else:
                print(f"Failed to archive {chat_id}: {resp.status_code}")
        except Exception as e:
            print(f"Error archiving {chat_id}: {e}")
            
    # Output archived IDs
    print("ARCHIVED_IDS:" + json.dumps(archived_ids))

if __name__ == "__main__":
    main()
