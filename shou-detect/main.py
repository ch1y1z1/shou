import mediapipe as mp
import cv2
import math
import requests

cap = cv2.VideoCapture(0)
mpHands = mp.solutions.hands

hands = mpHands.Hands()
mpDraw = mp.solutions.drawing_utils

api_url= 'http://192.168.31.183/api/set_angles'

def ccos(l1, l2, l3):
    def dis(p, q):
        return math.sqrt((p.x - q.x) ** 2 + (p.y - q.y) ** 2 + (p.z - q.z) ** 2)

    a = dis(l1, l2)
    b = dis(l2, l3)
    c = dis(l1, l3)
    return math.acos((a**2 + b**2 - c**2) / (2 * a * b))


while True:
    success, img = cap.read()
    imgRGB = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)
    results = hands.process(imgRGB)
    if results.multi_hand_landmarks:
        landmarks = results.multi_hand_landmarks[0].landmark
        c1 = ccos(landmarks[0], landmarks[5], landmarks[8])
        c2 = ccos(landmarks[0], landmarks[9], landmarks[12])
        c3 = ccos(landmarks[0], landmarks[13], landmarks[16])
        c4 = ccos(landmarks[0], landmarks[17], landmarks[20])
        print(c4)
        data = []
        data.append({"num": 1, "angle": int((c1 / math.pi) * 90)})
        data.append({"num": 2, "angle": int((c2 / math.pi) * 90)})
        data.append({"num": 3, "angle": int((c3 / math.pi) * 90)})
        data.append({"num": 4, "angle": int((c4 / math.pi) * 90)})
        requests.post(
            api_url,
            json={"data": data},
        )
